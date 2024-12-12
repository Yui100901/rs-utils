use crate::{command_utils, docker_utils, file_utils, git_utils};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use env_logger::builder;

/// 结构体定义: 存储仓库信息
#[derive(Debug, Serialize, Deserialize, Default)]
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
        match git_utils::clone_latest(&self.url, &self.branch, path) {
            Ok(s) => info!("{}", s),
            Err(e) => info!("{}", e),
        }
    }

    /// 拉取最新的仓库更改
    fn update(&self) {
        match git_utils::fetch() {
            Ok(s) => info!("{}", s),
            Err(e) => info!("{}", e),
        }
    }
}

/// 结构体定义: 存储构建器信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub repository: Repository,
    #[serde(default)]
    pub build_message: String,
    #[serde(skip_serializing,skip_deserializing)]
    pub builder_types:Vec<Box<dyn Builder>>,
}

trait Builder:Debug {
    fn build(&self) -> Result<String, Error>;
}
#[derive(Debug)]
struct Maven{
    path: String,
}

impl Maven {
    fn new(path: String) -> Self {
        Maven{path}
    }
}

impl Builder for Maven {
    /// 执行 Maven 构建
    fn build(&self) -> Result<String, Error>{
        info!("构建Maven项目");
        command_utils::run_command("mvn", &["clean", "package"])
    }
}

#[derive(Debug)]
struct Gradle{
    path: String,
}

impl Gradle {
    fn new(path: String) -> Self {
        Gradle{path}
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
struct Python{
    path: String
}

impl Python {
    fn new(path: String) -> Self {
        Python{path}
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
struct Node{
    path: String,
}

impl Node {
    fn new(path: String) -> Self {
        Node{path}
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
struct Go{
    path: String,
}

impl Go {
    fn new(path: String) -> Self {
        Go{path}
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
struct C{
    path: String,
}

impl C {
    fn new(path: String) -> Self {
        C{path}
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
struct Rust{
    path: String,
}

impl Rust {
    fn new(path: String) -> Self {
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
struct Docker{
    path:String,
    name:String
}

impl Docker {
    fn new(path:String,name:String)->Self{
        Docker{path,name}
    }
}

impl Builder for Docker {
    /// 执行 Docker 构建
    fn build(&self) -> Result<String, Error> {
        info!("构建Docker项目");
        docker_utils::build(&self.name)
    }
}


impl Project {
    /// 创建一个新的项目实例
    pub fn new(
        path: String,
        name: String,
        ports: Vec<String>,
        url: String,
        branch: String,
    ) -> Self {
        let repository = Repository::new(url, branch);
        if !Path::new(&path).exists() {
            //项目目录不存在
            match fs::create_dir_all(&path) {
                Ok(_) => info!("路径创建成功：{}", &path),
                Err(e) => error!("创建路径失败：{}", e),
            }
        }
        //项目目录存在
        if Path::new(&path).join(".git").exists() {
            //.git存在，进入项目目录，并获取最新代码
            info!("拉取最新代码");
            std::env::set_current_dir(&path).unwrap();
            repository.update();
        } else {
            //.git不存在
            if !repository.url.is_empty() {
                //项目地址不为空
                info!("克隆仓库 {}", &path);
                repository.clone(&path);
            }
        }
        let types=check_builder(path.as_str());
        let project = Project {
            path:path.clone(),
            name,
            ports,
            repository,
            build_message: String::new(),
            builder_types: types,
        };
        project.init_info();
        project
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
    pub fn get_source_code(&self) {

    }

    /// 构建项目
    pub fn build(&mut self) {
        std::env::set_current_dir(&self.path).unwrap();
        for builder_type in self.builder_types.iter() {
            builder_type.build().expect("构建出错");
        }

        info!("构建项目 {} 结束。", self.name);
        self.build_message = format!("{}", self.name);
    }

    pub fn deploy(&self){

    }
}

fn check_builder(path: &str) ->Vec<Box<dyn Builder>> {
    let path_str = path.to_string();
    let file_types:Vec<(&str,Box<dyn Fn() -> Box<dyn Builder>>)> = vec![
        ("pom.xml", Box::new(|| Box::new(Maven::new(path_str.clone())) as Box<dyn Builder>)),
        ("build.gradle", Box::new(|| Box::new(Gradle::new(path_str.clone())) as Box<dyn Builder>)),
        ("requirements.txt", Box::new(|| Box::new(Python::new(path_str.clone())) as Box<dyn Builder>)),
        ("package.json", Box::new(|| Box::new(Node::new(path_str.clone())) as Box<dyn Builder>)),
        ("go.mod", Box::new(|| Box::new(Go::new(path_str.clone())) as Box<dyn Builder>)),
        ("CMakeLists.txt", Box::new(|| Box::new(C::new(path_str.clone())) as Box<dyn Builder>)),
        ("Cargo.toml", Box::new(|| Box::new(Rust::new(path_str.clone())) as Box<dyn Builder>)),
        ("Dockerfile", Box::new(|| Box::new(Docker::new(path_str.clone(),"".to_string())) as Box<dyn Builder>)),
    ];
    let mut types: Vec<Box<dyn Builder>> = Vec::new();
    for (file_type, create_builder) in file_types {
        if Path::new(&format!("{}/{}", path, file_type)).exists() {
            info!("发现文件 {}。", file_type);
            types.push(create_builder());
        }
    }
    types
}
