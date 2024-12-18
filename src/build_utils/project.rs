use std::fs;
use std::path::Path;
use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::{docker_utils, git_utils};
use crate::build_utils::builder;

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
    #[serde(skip_serializing, skip_deserializing)]
    pub builder_vec: Vec<(String, Box<dyn builder::Builder>)>,
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
        let project = Project {
            path: path.clone(),
            name,
            ports,
            repository,
            build_message: String::new(),
            builder_vec: Vec::new(),
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
        if !Path::new(&self.path).exists() {
            //项目目录不存在
            match fs::create_dir_all(&self.path) {
                Ok(_) => info!("路径创建成功：{}", &self.path),
                Err(e) => error!("创建路径失败：{}", e),
            }
        }
        //项目目录存在
        if Path::new(&self.path).join(".git").exists() {
            //.git存在，进入项目目录，并获取最新代码
            info!("拉取最新代码");
            std::env::set_current_dir(&self.path).unwrap();
            self.repository.update();
        } else {
            //.git不存在
            if !self.repository.url.is_empty() {
                //项目地址不为空
                info!("克隆仓库 {}", &self.path);
                self.repository.clone(&self.path);
            }
        }
    }

    /// 初始化构建器
    pub fn init_builder(&mut self) {
        let path_str = self.path.to_string();
        let file_types: Vec<(&str, Box<dyn Fn() -> Box<dyn builder::Builder>>)> = vec![
            (
                "pom.xml",
                Box::new(|| {
                    Box::new(builder::Maven::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "build.gradle",
                Box::new(|| {
                    Box::new(builder::Gradle::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "requirements.txt",
                Box::new(|| {
                    Box::new(builder::Python::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "package.json",
                Box::new(|| {
                    Box::new(builder::Node::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "go.mod",
                Box::new(|| {
                    Box::new(builder::Go::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "CMakeLists.txt",
                Box::new(|| {
                    Box::new(builder::C::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "Cargo.toml",
                Box::new(|| {
                    Box::new(builder::Rust::new(path_str.clone())) as Box<dyn builder::Builder>
                }),
            ),
            (
                "Dockerfile",
                Box::new(|| {
                    Box::new(builder::Docker::new(
                        path_str.clone(),
                        self.name.to_string(),
                    )) as Box<dyn builder::Builder>
                }),
            ),
        ];
        for (file_type, create_builder) in file_types {
            if Path::new(&format!("{}/{}", path_str, file_type)).exists() {
                info!("发现文件 {}。", file_type);
                self.builder_vec
                    .push((file_type.to_string(), create_builder()));
            }
        }
    }

    /// 构建项目
    pub fn build(&mut self) {
        std::env::set_current_dir(&self.path).unwrap();
        if self.builder_vec.is_empty() {
            error!("没有找到任何可构建的文件！");
            return;
        }
        for (_, builder) in self.builder_vec.iter() {
            builder.build().expect("构建出错");
        }

        info!("构建项目 {} 结束。", self.name);
        self.build_message = format!("{}", self.name);
    }

    /// 部署到docker
    pub fn deploy_to_docker(&self) {
        if !self.builder_vec.iter().any(|(key, _)| key == "Dockerfile") {
            error!("项目{}没有对应的Dockerfile文件，无法部署！", self.name);
        }
        let port_list = self.ports.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        docker_utils::container_rerun(&self.name, &port_list).expect("启动docker容器出错!");
    }
}
