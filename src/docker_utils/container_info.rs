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
    fn parse_container_name(&self) -> String {
        self.Name.trim_start_matches('/').to_string()
    }
    fn parse_privileged(&self) -> bool {
        self.HostConfig.Privileged
    }
    fn parse_publish_all_ports(&self) -> bool {
        self.HostConfig.PublishAllPorts
    }
    fn parse_auto_remove(&self) -> bool {
        self.HostConfig.AutoRemove
    }
    fn parse_user(&self) -> String {
        if let Some(user) = &self.Config.User {
            if !user.is_empty() {
               return String::from(user);
            }
        }
        "".to_string()
    }
    fn parse_envs(&self) -> Vec<String> {
        if let Some(env_vars) = &self.Config.Env {
            return env_vars.clone();
        }
        Vec::new()
    }
    fn parse_mounts(&self) -> Vec<String> {
        let mut mounts=Vec::new();
        for mount in &self.Mounts {
            if !Path::new(&mount.Destination).is_absolute() {
                // 非绝对路径时挂载匿名卷
                mounts.push(mount.Destination.clone());
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
                mounts.push(volume);
            }
        }
        mounts
    }
    fn parse_port_bindings(&self) -> Vec<String> {
       let mut port_bindings=Vec::new();
        for (port, bindings) in &self.HostConfig.PortBindings {
            for binding in bindings {
                port_bindings.push(format!("{}:{}", binding.HostPort, port));
            }
        }
        port_bindings
    }
    fn parse_restart_policy(&self) -> String {
        format!("--restart={}", self.HostConfig.RestartPolicy.Name)
    }
    fn parse_image(&self) -> String {
        self.Config.Image.clone()
    }
}

pub struct DockerCommand{
    name: String,
    privileged: bool,
    publish_all_ports: bool,
    auto_remove: bool,
    restart_policy:String,
    user:String,
    envs:Vec<String>,
    mounts:Vec<String>,
    port_bindings:Vec<String>,
    image:String,
}

impl DockerCommand {
    pub fn from(info:ContainerInfo) ->Self{
        DockerCommand{
            name: info.parse_container_name(),
            privileged: info.parse_privileged(),
            publish_all_ports: info.parse_publish_all_ports(),
            auto_remove: info.parse_auto_remove(),
            restart_policy: info.parse_restart_policy(),
            user: info.parse_user(),
            envs: info.parse_envs(),
            mounts: info.parse_mounts(),
            port_bindings:info.parse_port_bindings(),
            image: info.parse_image(),
        }
    }

    pub fn to_command(&self)->Vec<String> {
        let mut command: Vec<String> =
            vec!["docker".to_string(), "run".to_string(), "-d".to_string()];
        //添加容器名称
        command.push("--name".to_string());
        command.push(self.name.clone());
        //是否添加高级权限
        if self.privileged {
            command.push("--privileged".to_string());
        }
        //是否映射所有端口
        if self.publish_all_ports {
            command.push("-P".to_string());
        }
        //是否自动移除
        if self.auto_remove {
            command.push("--rm".to_string());
        }
        //添加用户
        if !self.user.is_empty() {
            command.push("-u".to_string());
            command.push(self.user.to_string());
        }
        //添加环境变量
        for env in &self.envs {
            command.push("-e".to_string());
            command.push(env.to_string());
        }
        //添加挂载卷
        for mount in &self.mounts {
            command.push("-v".to_string());
            command.push(mount.to_string());
        }
        // 添加端口映射
        for port_binding in &self.port_bindings{
            command.push("-p".to_string());
            command.push(port_binding.to_string());
        }
        // 添加镜像名称
        command.push(self.image.clone());
        command
    }
}