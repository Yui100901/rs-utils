use log::{error, info, warn};

mod log_utils;
mod http_utils;

mod file_utils;
mod config;
mod command_utils;
mod git_utils;

#[tokio::main]
async fn main() {
    log_utils::init_logger();
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    let c=http_utils::HttpUtils::new();
    match file_utils::traverse_dir_files(".", false){
        Ok((files, dirs)) => {
            info!("Files:");
            for file in files {
                info!("{}", file.display());
            }

            info!("Directories:");
            for dir in dirs {
                info!("{}", dir.display());
            }
        }
        Err(e) => error!("Error:\n{}", e),
    }
    match command_utils::run_command("cmd", &["/c","echo","Hello, world!"]){
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match command_utils::run_command("cmd", &["/c","java","--version"]){
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match command_utils::run_command("java", &["--version"]){
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    match git_utils::clone_latest("https://github.com/Yui100901/rs-utils.git", "main","./rs-utils"){
        Ok(output) => info!("Output:\n{}", output),
        Err(e) => error!("Error:\n{}", e),
    };
    // // 示例GET请求
    // match c.get_by_query("https://www.baidu.com", None).await {
    //     Ok(response) => info!("GET响应: {}", response),
    //     Err(e) => error!("GET请求失败: {}", e),
    // }
}
