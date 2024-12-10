use crate::command_utils;
use log::{error, info};
use std::io::Error;
use std::process::Command;

pub fn docker_run_command(args: &[&str])-> Result<String, Error>{
    info!("执行自定义docker命令");
    command_utils::run_command("docker",args)
}

/// 停止docker容器
pub fn container_stop(containers: &[&str]) -> Result<String, Error> {
    info!("停止容器 {:?}", containers);
    let mut args = vec!["stop"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 强制停止docker容器
pub fn container_kill(containers: &[&str]) -> Result<String, Error> {
    info!("强制停止容器 {:?}", containers);
    let mut args = vec!["kill"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 删除docker容器
pub fn container_remove(containers: &[&str]) -> Result<String, Error> {
    info!("删除容器 {:?}", containers);
    let mut args = vec!["rm"];
    args.extend_from_slice(containers);
    command_utils::run_command("docker", &args)
}

/// 获取容器详细信息
pub fn container_inspect(name: &str) -> Result<String, Error> {
    info!("获取容器 {}详细信息", name);
    let mut args = vec!["inspect"];
    args.push(name);
    command_utils::run_command("docker", &args)
}

/// 获取docker镜像列表
pub fn image_list_formatted() -> Result<String, Error> {
    info!("列出格式化的镜像列表");
    let args = vec!["images", "--format", "{{.Repository}}:{{.Tag}}"];
    command_utils::run_command("docker", &args)
}

/// 删除docker镜像
pub fn image_remove(images: &[&str]) -> Result<String, Error> {
    info!("删除镜像 {:?}", images);
    let mut args = vec!["rmi"];
    args.extend_from_slice(images);
    command_utils::run_command("docker", &args)
}

/// 构建Docker镜像
pub fn build(name: &str) -> Result<String, Error> {
    info!("构建镜像 {}", name);
    let args =  vec!["build", "-t", name, "."];
    command_utils::run_command("docker", &args)
}

/// 导出Docker镜像
pub fn save(name: &str,path:&str) -> Result<String, Error> {
    info!("导出镜像 {}", name);
    let filename = format!("{}/{}.tar", path,name.replace(':', "_").replace('/', "_"));
    let args =vec!["save", "-o", &filename, name];
    command_utils::run_command("docker", &args)
}

/// 导入Docker镜像
pub fn load(path: &str) -> Result<String, Error> {
    info!("导入镜像 {}", path);
    let args = vec!["load", "-i", path];
    command_utils::run_command("docker", &args)
}

/// 清理docker镜像
pub fn image_prune() -> Result<String, Error> {
    info!("清理镜像");
    let args = vec!["image", "prune", "-f"];
    command_utils::run_command("docker", &args)
}

/// 默认启动Docker容器
pub fn default_run(name: &str, ports: &[&str]) -> Result<String, Error> {
    info!("默认启动 {:?}", name);
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
        ports_mappings.push(String::from("-p"));
        ports_mappings.push(format!("{}:{}", p, p));
    }
    args.append(&mut ports_mappings);
    args.push(format!("{}:latest", name));
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    command_utils::run_command("docker", &args_ref)
}

/// 重新创建Docker容器
pub fn container_rerun(name: &str, ports: &[&str]) -> Result<String, Error> {
    container_stop(&[name])?;
    container_remove(&[name])?;
    default_run(name, ports)
}

