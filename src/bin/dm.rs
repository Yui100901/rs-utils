use std::collections::HashMap;
use std::io::{BufRead, Error};
use std::path::Path;
use std::process::Command;
use clap::{Parser, Subcommand};
use log::{error, info};
use serde::Deserialize;
use rs_utils::{docker_utils, file_utils, log_utils};

#[derive(Parser,Debug)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Build {
        #[arg(short, long)]
        export: bool,

        path: String,
    },
    Recreate {
        path: String,
    },
    Clean{
    },
    Import {
        path: Option<String>,
    },
    Export {
        path: Option<String>,
    },
    Reverse{
        name:String,
    }
}

fn main() {
    log_utils::init_logger();
    let cli = Cli::parse();
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Build {export,path} => {
                if !Path::new(&path).join("Dockerfile").exists() {
                    error!("Dockerfile does not exists!");
                    return;
                }
                build(&path, export).expect("构建失败");
            },
            Commands::Recreate {path} => {

            },
            Commands::Clean {} =>{
                clean().expect("Failed to clean containers!");
            },
            Commands::Import {path} => {
                if let Some(path) = path {
                    import(&path).expect("Import failed!");
                }else {
                    import("images").expect("Import failed!");
                }
            }
            Commands::Export {path} => {
                if let Some(path) = path {
                    file_utils::create_directory(&path).expect("Create directory failed!");
                    export(&path).expect("Export failed!");
                }else {
                    file_utils::create_directory("images").expect("Create directory failed!");
                    export("images").expect("Export failed!");
                }
            }
            Commands::Reverse {name } => {
                match reverse(&name) {
                    Ok(cmd) => {info!("Generated docker command:\n{}",cmd.as_str())}
                    Err(e) => {error!("Error to reverse container:{}",e)}
                }
            }
        }
    }
}

fn build(path: &str,export: bool) -> Result<String, Error>{
    let file_data =file_utils::file_data::FileData::new(path.to_string()).unwrap();
    std::env::set_current_dir(&file_data.abs_path)?;
    docker_utils::build(&file_data.filename)?;
    if export {
        docker_utils::save(&file_data.filename,".")?;
    }
    Ok("".to_string())
}

fn clean()-> Result<String, Error> {
    docker_utils::image_prune()
}

fn import(path: &str) -> Result<String, Error> {
    match file_utils::file_data::FileData::new(path.to_string()){
        Ok(data) => {
            match file_utils::traverse_dir_files(data.abs_path.as_str(),true){
                Ok((_,files)) => {
                    for file in files{
                        docker_utils::load(file.path.as_str())?;
                    }
                },
                Err(e)=>{
                    error!("Traverse {} failed!Error:{}", path,e);
                }
            }
        },
        Err(e)=>{
            error!("Get path {} failed!Error:{}", path,e);
        }
    }
    Ok("".to_string())
}

fn export(path: &str) -> Result<String, Error> {
    docker_utils::image_prune()?;
    let images=docker_utils::image_list_formatted()?;
    let images: Vec<&str> = images.lines().filter(|line| !line.is_empty()).collect();
    for image in images{
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
struct HostConfig {
    PortBindings: HashMap<String, Vec<PortBinding>>,
}

#[derive(Deserialize, Debug)]
struct Config {
    Env: Option<Vec<String>>,
    Cmd: Option<Vec<String>>,
    Image: String,
}

#[derive(Deserialize, Debug)]
struct ContainerInfo {
    Config: Config,
    HostConfig: HostConfig,
    Mounts: Vec<Mount>,
}

fn reverse(name:&str) -> Result<String, Error> {
    match docker_utils::container_inspect(name){
        Ok(data) => {
            let container_info: Vec<ContainerInfo> = serde_json::from_str(data.as_str())?;
            let container_info= container_info.into_iter().next().unwrap();
            let mut command:Vec<String> = vec![
                "docker".to_string(),
                "run".to_string(),
                "-d".to_string(),
                "--name".to_string(),
                name.to_string(),
            ];
            // 添加环境变量
            if let Some(env_vars) = &container_info.Config.Env {
                for env in env_vars {
                    command.push(format!("-e {}", env));
                }
            }
            // 添加挂载卷
            for mount in &container_info.Mounts {
                let mut volume_str=String::new();
                // 检查并修复绝对路径
                if !Path::new(&mount.Destination).is_absolute() {
                    volume_str = format!("-v {}",mount.Destination);
                }else {
                    volume_str = if mount.Mode.is_empty() {
                        format!("-v {}:{}:rw", mount.Source, mount.Destination)
                    } else {
                        format!("-v {}:{}:{}", mount.Source, mount.Destination, mount.Mode)
                    };

                }
                command.push(volume_str);
            }
            // 添加端口映射
            for (port, bindings) in &container_info.HostConfig.PortBindings {
                for binding in bindings {
                    command.push(format!("-p {}:{}", binding.HostPort, port));
                }
            }
            // 添加镜像名称
            command.push(container_info.Config.Image.clone());
            // // 添加其他配置信息
            // if let Some(cmd) = &container_info.Config.Cmd {
            //     let cmd_str = cmd.join(" ");
            //     command.push(format!("-- {}", cmd_str));
            // }
            Ok(command.join(" "))
        }
        Err(e) => {
            error!("Failed to inspect container {}: {}", name, e);
            Err(e)
        }
    }
}