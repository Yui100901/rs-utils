use crate::http_utils::HttpUtils;
use log::{error, info, warn};
use reqwest::Client;
use serde::Serialize;

mod http_utils;
mod log_utils;

mod build_utils;
mod command_utils;
mod config;
mod docker_utils;
mod file_utils;
mod git_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_utils::init_logger();
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    let c = http_utils::HttpUtils::new();
    match file_utils::traverse_dir_files(".", false) {
        Ok((files, dirs)) => {
            info!("Files:");
            for file in files {
                info!("{}", file.path_buf.display());
            }

            info!("Directories:");
            for dir in dirs {
                info!("{}", dir.path_buf.display());
            }
        }
        Err(e) => error!("Error:\n{}", e),
    }
    match command_utils::run_command("cmd", &["/c", "echo", "Hello, world!"]) {
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match command_utils::run_command("cmd", &["/c", "java", "--version"]) {
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match command_utils::run_command("java", &["--version"]) {
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match git_utils::clone_latest(
        "https://github.com/Yui100901/rs-utils.git",
        "main",
        "./rs-utils",
    ) {
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    // let mut b = build_utils::builder::Builder::new(
    //     "C:\\Users\\yfy2001\\yfy\\Learn\\Projects\\project-builder",
    //     "rs-utils",
    //     "",
    //     "",
    // );
    // b.build();
    // build_utils::builder::Builder::new()
    // let api_url = "http://42.192.69.243:20379";
    // let api_url = "http://127.0.0.1:21011/routePlan";
    // // let api_url = "https://www.baidu.com";
    // // 示例GET请求
    // match c.get_by_query(api_url, None, None).await {
    //     Ok(response) => info!("GET响应: {}", response),
    //     Err(e) => error!("GET请求失败: {:?}", e),
    // }

    #[derive(Serialize)]
    struct Obstacle {
        center: [f64; 2],
        edge_lengths: [i32; 2],
    }

    #[derive(Serialize)]
    struct RoutePlanRequest {
        start_point: [f64; 2],
        end_point: [f64; 2],
        obstacle_params: Vec<Obstacle>,
    }
    match command_utils::run_command("cmd", &["/c","dir"]){
        Ok(output) => info!("run_command Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    }
    Ok(())
}
