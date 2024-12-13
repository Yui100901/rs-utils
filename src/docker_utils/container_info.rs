use std::collections::HashMap;
use std::io::Error;
use std::path::Path;
use serde::Deserialize;

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
    AutoRemove: bool,
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
pub struct ContainerInfo {
    pub Name: String,
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
        //是否自动移除
        if self.HostConfig.AutoRemove {
            command.push("--rm".to_string());
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