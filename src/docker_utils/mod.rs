use std::io::Error;
use std::process::Command;
use log::{info, error};
use crate::command_utils;

/// 停止docker容器
pub fn stop_container(containers: &[&str]) -> Result<String, Error> {
    info!("停止docker容器");
    let mut args = vec!["stop"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 强制停止docker容器
pub fn kill_container(containers: &[&str]) -> Result<String, Error> {
    info!("强制停止docker容器");
    let mut args = vec!["kill"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 删除docker容器
pub fn remove_container(containers: &[&str]) -> Result<String, Error> {
    info!("删除docker容器");
    let mut args = vec!["rm"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 删除docker镜像
pub fn remove_image(images: &[&str]) -> Result<String, Error> {
    info!("删除docker镜像");
    let mut args = vec!["rmi"];
    args.extend_from_slice(images);
    command_utils::run_command("docker", &args)
}

/// 构建Docker镜像
pub fn build(name: &str) -> Result<String, Error> {
    info!("构建Docker镜像");
    command_utils::run_command("docker", &["build", "-t", name, "."])
}

/// 默认启动Docker容器
pub fn default_run(name: &str, ports: &[&str]) -> Result<String, Error> {
    info!("默认启动");
    let mut args = vec!["run", "-d", "--name", name, "-v", "/etc/localtime:/etc/localtime:ro"];
    for p in ports {
        args.push("-p");
        args.push(&format!("{}:{}", p, p));
    }
    args.push(&format!("{}:latest", name));
    command_utils::run_command("docker", &args)
}

/// 重新构建Docker容器
pub fn rebuild_container(name: &str, ports: &[&str]) -> Result<String, Error> {
    stop_container(&[name])?;
    remove_container(&[name])?;
    remove_image(&[name])?;
    build(name)?;
    default_run(name, ports)
}

