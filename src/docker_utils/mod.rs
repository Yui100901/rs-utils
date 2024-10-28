use crate::command_utils;
use log::{error, info};
use std::io::Error;
use std::process::Command;

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
    let mut args: Vec<String> = vec![
        "run".into(),
        "-d".into(),
        "--name".into(),
        name.into(),
        "-v".into(),
        "/etc/localtime:/etc/localtime:ro".into(),
    ];
    let mut ports_mappings: Vec<String> = Vec::new();
    for p in ports {
        ports_mappings.push(format!("{}:{}", p, p));
    }
    args.append(&mut ports_mappings);
    args.push(format!("{}:latest", name));
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    command_utils::run_command("docker", &args_ref)
}

/// 重新构建Docker容器
pub fn rebuild_container(name: &str, ports: &[&str]) -> Result<String, Error> {
    stop_container(&[name])?;
    remove_container(&[name])?;
    remove_image(&[name])?;
    build(name)?;
    default_run(name, ports)
}
