use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use log::info;
use crate::{command_utils, docker_utils, file_utils, git_utils};

/// 结构体定义: 存储仓库信息
#[derive(Debug, Serialize,Deserialize,Default)]
pub struct Repository {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub branch: String,
}

impl Repository {
    /// 创建一个新的仓库实例
    fn new(url: String, branch: String) -> Self {
        Repository { url, branch }
    }

    /// 克隆仓库到指定路径
    pub fn clone(&self, path: &str) {
        match git_utils::clone_latest(&self.url,&self.branch,path){
            Ok(s)=>info!("{}",s),
            Err(e)=>info!("{}",e),
        }
    }

    /// 拉取最新的仓库更改
    fn pull(&self)  {
        match git_utils::pull(){
            Ok(s)=>info!("{}",s),
            Err(e)=>info!("{}",e),
        }
    }
}

/// 结构体定义: 存储构建器信息
#[derive(Debug, Serialize,Deserialize,Default)]
pub struct Builder {
    #[serde(default)]
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub repository: Repository,
    #[serde(default)]
    pub build_message: String,
}

impl Builder {
    /// 创建一个新的构建器实例
    pub fn new(
        path: String,
        name: String,
        ports: Vec<String>,
        url: String,
        branch: String,
    ) -> Self {
        let repository = Repository::new(url, branch);
        let builder = Builder {
            path,
            name,
            ports,
            repository,
            build_message: String::new(),
        };
        builder.init_info();
        builder
    }

    /// 初始化构建器信息
    fn init_info(&self) {
        info!("初始化构建器！");
        info!("项目路径：{}，项目名：{}", self.path, self.name);
        info!(
            "项目地址：{}，项目分支：{}",
            self.repository.url, self.repository.branch
        );
    }

    /// 克隆或拉取仓库
    pub fn clone_repository(&self, force_clone: bool) {
        if Path::new(&self.path).exists() {
            if force_clone {
                info!("目录 {} 已存在，删除并重新克隆。", self.path);
                fs::remove_dir_all(&self.path).unwrap();
                self.repository.clone(&self.path);
            } else {
                self.repository.pull();
                info!("目录 {} 已存在，跳过克隆。", self.path);
            }
        } else {
            self.repository.clone(&self.path);
        }
    }

    /// 构建项目
    pub fn build(&mut self) {
        std::env::set_current_dir(&self.path).unwrap();
        let build_commands: HashMap<&str, fn() -> Result<String, Error>> = vec![
            ("pom.xml", maven_build as fn() -> Result<String, Error>),
            ("build.gradle", gradle_build),
            ("requirements.txt", python_build),
            ("package.json", node_build),
            ("go.mod", go_build),
            ("CMakeLists.txt", c_build),
            ("Cargo.toml", rust_build),
        ]
        .into_iter()
        .collect();

        let mut build_flag = 0;

        for (file, build) in build_commands {
            if Path::new(file).exists() {
                build_flag += 1;
                if let Err(err) = build() {
                    self.build_message = format!("{}项目构建出错:\n{}", self.name, err);
                    return;
                }
                break;
            }
        }

        if Path::new("Dockerfile").exists() {
            build_flag += 1;
            // 先将 Vec<String> 转换为 Vec<&str>
            let ports_vec: Vec<&str> = self.ports.iter().map(|s| s.as_str()).collect();
            // 将 Vec<&str> 转换为切片 &[&str]
            let ports_slice = Box::leak(ports_vec.into_boxed_slice());
            if let Err(err) = docker_utils::rebuild_container(&self.name, ports_slice) {
                println!("Docker构建失败: {}", err);
            } else {
                println!("Docker构建成功");
            }
        }

        let output_info = if build_flag == 0 {
            "未找到构建的文件！".to_string()
        } else {
            "构建成功！".to_string()
        };

        println!("构建项目 {} 结束。{}", self.name, output_info);
        self.build_message = format!("{}{}", self.name, output_info);
    }
}

/// 执行 Maven 构建
fn maven_build() -> Result<String, Error> {
    println!("构建Maven项目");
    command_utils::run_command("mvn", &["clean", "package"])
}

/// 执行 Gradle 构建
fn gradle_build() -> Result<String, Error> {
    println!("构建Gradle项目");
    command_utils::run_command("gradle", &["build"])
}

/// 执行 Python 构建
fn python_build() -> Result<String, Error> {
    println!("构建Python项目");
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

/// 执行 Node.js 构建
fn node_build() -> Result<String, Error> {
    println!("构建Node项目");
    command_utils::run_command(
        "npm",
        &["install", "--registry=https://registry.npmmirror.com"],
    )?;
    command_utils::run_command("npm", &["run", "build"])?;
    let work_dir = std::env::current_dir().unwrap();
    let source = Path::new("/media/zmkj/work/node_file/Cesium.js");
    let target = work_dir.join("dist/cesium/Cesium.js");
    file_utils::replace(&source, &target)
}

/// 执行 Go 构建
fn go_build() -> Result<String, Error> {
    println!("构建Go项目");
    command_utils::run_command("go", &["env", "-w", "GO111MODULE=on"])?;
    command_utils::run_command("go", &["env", "-w", "GOPROXY=https://goproxy.cn,direct"])?;
    command_utils::run_command("go", &["build"])
}

/// 执行 C 构建
fn c_build() -> Result<String, Error> {
    println!("构建C项目");
    command_utils::run_command("cmake", &[".."])?;
    command_utils::run_command("make", &[])
}

/// 执行 Rust 构建
fn rust_build() -> Result<String, Error> {
    println!("构建Rust项目");
    command_utils::run_command("cargo", &["build","--release"])
}
