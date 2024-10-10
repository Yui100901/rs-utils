use crate::{git_utils, log_utils,command_utils};
use log::{error, info};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// 表示一个项目仓库
struct Repository {
    url: String,    // 项目仓库的 URL
    branch: String, // 分支
}

impl Repository {
    /// 创建一个新的 Repository 实例
    fn new(url: &str, branch: &str) -> Self {
        Self {
            url: url.to_string(),
            branch: branch.to_string(),
        }
    }

    /// 克隆仓库到指定路径
    fn clone(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        git_utils::clone_latest(&self.url, &self.branch, path)?;
        Ok(())
    }
}

/// 表示一个项目构建器
pub struct Builder {
    project_path: String,           // 项目存放路径
    project_name: String,           // 项目名称
    project_repository: Repository, // 项目仓库
    build_message: String,          // 构建信息
}

impl Builder {
    /// 创建一个新的 Builder 实例
    pub fn new(
        project_path: &str,
        project_name: &str,
        project_url: &str,
        project_branch: &str,
    ) -> Self {
        info!("初始化构建器！");
        info!("项目路径：{} 项目名：{}", project_path, project_name);
        info!("项目地址：{} 项目分支：{}", project_url, project_branch);

        Self {
            project_path: project_path.to_string(),
            project_name: project_name.to_string(),
            project_repository: Repository::new(project_url, project_branch),
            build_message: String::new(),
        }
    }

    /// 克隆项目仓库
    pub fn clone_repository(&self, force_clone: bool) {
        if Path::new(&self.project_path).exists() {
            if force_clone {
                info!("目录 {} 已存在，删除并重新克隆。", self.project_path);
                fs::remove_dir_all(&self.project_path).unwrap();
                self.project_repository.clone(&self.project_path).unwrap();
            } else {
                info!("目录 {} 已存在，跳过克隆。", self.project_path);
            }
        } else {
            self.project_repository.clone(&self.project_path).unwrap();
        }
    }

    /// 构建项目
    pub fn build(&mut self) {
        env::set_current_dir(&self.project_path).unwrap();
        let build_commands: Vec<(&str, fn() -> Result<(), Box<dyn std::error::Error>>)> = vec![
            ("pom.xml", maven_build),
            ("build.gradle", gradle_build),
            ("requirements.txt", python_build),
            ("package.json", node_build),
            ("go.mod", go_build),
            ("CMakeLists.txt", c_build),
            ("Cargo.toml", rust_build),
        ];

        let mut build_flag = 0;

        for (file, build) in build_commands {
            if Path::new(file).exists() {
                build_flag += 1;
                if let Err(e) = build() {
                    self.build_message = format!("{}项目构建出错:\n{}", self.project_name, e);
                    return;
                }
                break;
            }
        }

        if Path::new("Dockerfile").exists() {
            build_flag += 1;
            command_utils::run_command(
                "docker",
                &["build", "-t", &format!("{}:latest", self.project_name), "."],
            )
            .unwrap();
        }

        let output_info = if build_flag == 0 {
            "未找到构建的文件！".to_string()
        } else {
            "构建成功！".to_string()
        };

        info!("构建项目 {} 结束。{}", self.project_name, output_info);
        self.build_message = format!("{}{}", self.project_name, output_info);
    }
}

/// 构建 Maven 项目
fn maven_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建Maven项目");
    command_utils::run_command("mvn", &["clean", "package"])?;
    Ok(())
}

/// 构建 Gradle 项目
fn gradle_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建Gradle项目");
    command_utils::run_command("gradle", &["build"])?;
    Ok(())
}

/// 构建 Python 项目
fn python_build() -> Result<(), Box<dyn std::error::Error>> {
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
    )?;
    Ok(())
}

/// 构建 Node 项目
fn node_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建Node项目");
    command_utils::run_command(
        "npm",
        &["install", "--registry=https://registry.npmmirror.com"],
    )?;
    command_utils::run_command("npm", &["run", "build"])?;
    Ok(())
}

/// 构建 Go 项目
fn go_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建Go项目");
    command_utils::run_command("go", &["env", "-w", "GO111MODULE=on"])?;
    command_utils::run_command("go", &["env", "-w", "GOPROXY=https://goproxy.cn,direct"])?;
    command_utils::run_command("go", &["build"])?;
    Ok(())
}

/// 构建 C 项目
fn c_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建C项目");
    command_utils::run_command("cmake", &[".."])?;
    command_utils::run_command("make", &[])?;
    Ok(())
}

/// 构建 Rust 项目
fn rust_build() -> Result<(), Box<dyn std::error::Error>> {
    info!("构建Rust项目");
    command_utils::run_command("cargo", &["build"])?;
    Ok(())
}




