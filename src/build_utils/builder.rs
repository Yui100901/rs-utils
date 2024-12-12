use crate::{command_utils, docker_utils, file_utils};
use log::info;
use std::fmt::Debug;
use std::io::Error;
use std::path::Path;

pub(crate) trait Builder: Debug {
    fn build(&self) -> Result<String, Error>;
}
#[derive(Debug)]
pub(crate) struct Maven {
    path: String,
}

impl Maven {
    pub(crate) fn new(path: String) -> Self {
        Maven { path }
    }
}

impl Builder for Maven {
    /// 执行 Maven 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Maven项目");
        command_utils::run_command("mvn", &["clean", "package"])
    }
}

#[derive(Debug)]
pub(crate) struct Gradle {
    path: String,
}

impl Gradle {
    pub(crate) fn new(path: String) -> Self {
        Gradle { path }
    }
}

impl Builder for Gradle {
    /// 执行 Gradle 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Gradle项目");
        command_utils::run_command("gradle", &["build"])
    }
}

#[derive(Debug)]
pub(crate) struct Python {
    path: String,
}

impl Python {
    pub(crate) fn new(path: String) -> Self {
        Python { path }
    }
}

impl Builder for Python {
    /// 执行 Python 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Python项目");
        command_utils::run_command(
            "pip",
            &[
                "install",
                "-r",
                "requirements.txt",
                "-i",
                "https://pypi.tuna.tsinghua.edu.cn/simple",
            ],
        )
    }
}

#[derive(Debug)]
pub(crate) struct Node {
    path: String,
}

impl Node {
    pub(crate) fn new(path: String) -> Self {
        Node { path }
    }
}

impl Builder for Node {
    /// 执行 Node.js 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Node项目");
        command_utils::run_command(
            "npm",
            &["install", "--registry=https://registry.npmmirror.com"],
        )?;
        command_utils::run_command("npm", &["run", "build"])?;
        let work_dir = std::env::current_dir()?;
        let source = Path::new("/root/node_file/Cesium.js");
        let target = work_dir.join("dist/cesium/Cesium.js");
        if source.exists() && target.parent().map_or(false, |p| p.exists()) {
            file_utils::replace(&source, &target).expect("替换文件失败");
            println!("文件替换成功！");
        } else {
            if !source.exists() {
                println!("源文件不存在：{}", source.display());
            }
            if !target.parent().map_or(false, |p| p.exists()) {
                println!("目标目录不存在：{}", target.display());
            }
        }
        Ok(String::from(""))
    }
}

#[derive(Debug)]
pub(crate) struct Go {
    path: String,
}

impl Go {
    pub(crate) fn new(path: String) -> Self {
        Go { path }
    }
}

impl Builder for Go {
    /// 执行 Go 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Go项目");
        command_utils::run_command("go", &["env", "-w", "GO111MODULE=on"])?;
        command_utils::run_command("go", &["env", "-w", "GOPROXY=https://goproxy.cn,direct"])?;
        command_utils::run_command("go", &["build"])
    }
}

#[derive(Debug)]
pub(crate) struct C {
    path: String,
}

impl C {
    pub(crate) fn new(path: String) -> Self {
        C { path }
    }
}

impl Builder for C {
    /// 执行 C 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建C项目");
        command_utils::run_command("cmake", &[".."])?;
        command_utils::run_command("make", &[])
    }
}

#[derive(Debug)]
pub(crate) struct Rust {
    path: String,
}

impl Rust {
    pub(crate) fn new(path: String) -> Self {
        Rust { path }
    }
}

impl Builder for Rust {
    /// 执行 Rust 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Rust项目");
        command_utils::run_command("cargo", &["build", "--release"])
    }
}

#[derive(Debug)]
pub(crate) struct Docker {
    path: String,
    name: String,
}

impl Docker {
    pub(crate) fn new(path: String, name: String) -> Self {
        Docker { path, name }
    }
}

impl Builder for Docker {
    /// 执行 Docker 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Docker项目");
        docker_utils::build(&self.name)
    }
}
