use clap::{CommandFactory, Parser, Subcommand};
use log::{error, info, warn};
use rs_utils::command_utils::run_command;
use rs_utils::{docker_utils, file_utils, log_utils};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, Error};
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, author="Yui100901", about="Docker小工具，可用于管理容器。", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "构建Docker镜像")]
    Build {
        #[arg(short, long, help = "构建完成后导出镜像")]
        export: bool,
        #[arg(help = "Dockerfile所在路径")]
        path: String,
    },
    Rerun {
        path: String,
    },
    #[command(about = "清理Docker镜像")]
    Clean {},
    #[command(about = "清理Docker镜像并导出所有镜像")]
    Import {
        #[arg(help = "默认导出至当前目录下的images")]
        path: Option<String>,
    },
    #[command(about = "导入Docker镜像")]
    Export {
        #[arg(help = "默认从当前目录下的images寻找镜像")]
        path: Option<String>,
    },
    #[command(about = "逆向Docker容器到启动命令")]
    Reverse {
        #[arg(short, long, help = "逆向解析完成后以解析出的命令重新创建容器")]
        rerun: bool,
        #[arg(help = "容器ID或名称")]
        names: Vec<String>,
    },
}

fn main() {
    log_utils::init_logger();
    let cli = Cli::parse();
    // 如果没有输入任何子命令，显示帮助信息
    if cli.command.is_none() {
        Cli::command().print_help().unwrap();
        std::process::exit(0);
    }
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Build { export, path } => {
                if !Path::new(&path).join("Dockerfile").exists() {
                    error!("Dockerfile does not exists!");
                    return;
                }
                build(&path, export).expect("构建失败");
            }
            Commands::Rerun { path } => {}
            Commands::Clean {} => {
                clean().expect("Failed to clean containers!");
            }
            Commands::Import { path } => {
                let path = path.unwrap_or_else(|| "images".to_string());
                import(&path).expect("Import failed!");
            }
            Commands::Export { path } => {
                let path = path.unwrap_or_else(|| "images".to_string());
                file_utils::create_directory(&path).expect("Create directory failed");
                export(&path).expect("Export failed");
            }
            Commands::Reverse { rerun, names } => {
                let container_names:Vec<&str>=names.iter().map(AsRef::as_ref).collect();
                match reverse(&container_names) {
                    Ok(cmds) => {
                        // warn!("{:?}",cmd);
                        let mut file = File::create("docker_commands.sh").unwrap();
                        writeln!(file, "#!/bin/bash").expect("Failed to write file!");
                        for (name,cmd) in cmds {
                            writeln!(file, "# {}", name).expect("Failed to write file!");
                            writeln!(file, "{}", cmd.join(" ")).expect("Failed to write file!");
                            info!("Generated docker command:\n{}", cmd.join(" "));
                            if rerun {
                                docker_utils::container_stop(&[name.as_str()]).unwrap();
                                docker_utils::container_remove(&[name.as_str()]).unwrap();
                                let args: Vec<&str> = cmd[1..].iter().map(AsRef::as_ref).collect();
                                docker_utils::docker_run_command(&args)
                                    .expect("Docker command failed!");
                            }
                        }
                        info!("Save command to docker_commands.sh successfully!");
                    }
                    Err(e) => {
                        error!("Error to reverse container:{}", e)
                    }
                }
            }
        }
    }
}

fn build(path: &str, export: bool) -> Result<String, Error> {
    let file_data = file_utils::file_data::FileData::new(path.to_string()).unwrap();
    std::env::set_current_dir(&file_data.abs_path)?;
    docker_utils::build(&file_data.filename)?;
    if export {
        docker_utils::save(&file_data.filename, ".")?;
    }
    Ok("".to_string())
}

fn clean() -> Result<String, Error> {
    docker_utils::image_prune()
}

fn import(path: &str) -> Result<String, Error> {
    match file_utils::file_data::FileData::new(path.to_string()) {
        Ok(data) => match file_utils::traverse_dir_files(data.abs_path.as_str(), true) {
            Ok((_, files)) => {
                for file in files {
                    docker_utils::load(file.path.as_str())?;
                }
            }
            Err(e) => {
                error!("Traverse {} failed!Error:{}", path, e);
            }
        },
        Err(e) => {
            error!("Get path {} failed!Error:{}", path, e);
        }
    }
    Ok("".to_string())
}

fn export(path: &str) -> Result<String, Error> {
    docker_utils::image_prune()?;
    let images = docker_utils::image_list_formatted()?;
    let images: Vec<&str> = images.lines().filter(|line| !line.is_empty()).collect();
    for image in images {
        if let Err(e) = docker_utils::save(image, path) {
            error!("Failed to save image {}: {}", image, e);
        }
    }
    Ok("Export success!".to_string())
}

#[derive(Deserialize, Debug)]
struct Mount {
    Source: String,
    Destination: String,
    Mode: String,
}

#[derive(Deserialize, Debug)]
struct PortBinding {
    HostPort: String,
}

#[derive(Deserialize, Debug)]
struct RestartPolicy {
    Name: String,
}

#[derive(Deserialize, Debug)]
struct HostConfig {
    PortBindings: HashMap<String, Vec<PortBinding>>,
    RestartPolicy: RestartPolicy,
    Privileged: bool,
    PublishAllPorts: bool,
}

#[derive(Deserialize, Debug)]
struct Config {
    User: Option<String>,
    Env: Option<Vec<String>>,
    Cmd: Option<Vec<String>>,
    Image: String,
}

#[derive(Deserialize, Debug)]
struct ContainerInfo {
    Name: String,
    Config: Config,
    HostConfig: HostConfig,
    Mounts: Vec<Mount>,
}

impl ContainerInfo {
    pub fn to_shell_command(&self) -> Result<Vec<String>, Error> {
        let mut command: Vec<String> =
            vec!["docker".to_string(), "run".to_string(), "-d".to_string()];
        //添加权限
        if self.HostConfig.Privileged {
            command.push("--privileged".to_string());
        }
        //映射所有端口
        if self.HostConfig.PublishAllPorts {
            command.push("-P".to_string());
        }
        //添加重启策略
        command.push(format!("--restart={}", self.HostConfig.RestartPolicy.Name));
        //添加容器名称
        command.push("--name".to_string());
        command.push(self.Name.trim_start_matches('/').to_string());
        //添加用户
        if let Some(user) = &self.Config.User {
            if !user.is_empty() {
                command.push("-u".to_string());
                command.push(user.to_string());
            }
        }
        // 添加环境变量
        if let Some(env_vars) = &self.Config.Env {
            for env in env_vars {
                command.push("-e".to_string());
                command.push(env.to_string());
            }
        }
        // 添加挂载卷
        for mount in &self.Mounts {
            command.push("-v".to_string());
            if !Path::new(&mount.Destination).is_absolute() {
                // 非绝对路径时挂载匿名卷
                command.push(mount.Destination.clone());
            } else {
                let volume = format!(
                    "{}:{}{}",
                    mount.Source,
                    mount.Destination,
                    if mount.Mode.is_empty() {
                        "".to_string()
                    } else {
                        format!(":{}", mount.Mode)
                    }
                );
                command.push(volume);
            }
        }
        // 添加端口映射
        for (port, bindings) in &self.HostConfig.PortBindings {
            for binding in bindings {
                command.push("-p".to_string());
                command.push(format!("{}:{}", binding.HostPort, port));
            }
        }
        // 添加镜像名称
        command.push(self.Config.Image.clone());
        // // 添加其他配置信息
        // if let Some(cmd) = &container_info.Config.Cmd {
        //     let cmd_str = cmd.join(" ");
        //     command.push(format!("-- {}", cmd_str));
        // }
        Ok(command)
    }
}

fn reverse(names: &[&str]) -> Result<HashMap<String,Vec<String>>, Error> {
    match docker_utils::container_inspect(names) {
        Ok(data) => {
            let container_info_list: Vec<ContainerInfo> = serde_json::from_str(data.as_str())?;
            let mut command_map: HashMap<String,Vec<String>> = HashMap::new();
            for container_info in container_info_list{
                match container_info.to_shell_command() {
                    Ok(command) => {
                        command_map.insert(container_info.Name, command);
                    },
                    Err(e) => {
                        error!("{}",e);
                        continue;
                    }
                }
            }
            Ok(command_map)
        }
        Err(e) => {
            error!("Failed to inspect container {:?}: {}", names, e);
            Err(e)
        }
    }
}
