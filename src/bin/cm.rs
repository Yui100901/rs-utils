use std::io::{BufRead, Error};
use clap::{Parser, Subcommand};
use rs_utils::{docker_utils, file_utils};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        export: bool,

        /// 端口列表
        #[arg(short, long)]
        ports: Option<String>,

        path: String,
    },
    Redeploy {
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Build {export,ports,path} => {
                let port_list: Vec<String> = ports
                    .unwrap_or_else(|| "".to_string())
                    .split(',')
                    .map(|s| s.to_string()).collect();
                // 先将 Vec<String> 转换为 Vec<&str>
                let ports_vec: Vec<&str> = port_list.iter().map(|s| s.as_str()).collect();
                // 将 Vec<&str> 转换为切片 &[&str]
                let ports_slice = Box::leak(ports_vec.into_boxed_slice());
                build(&path, ports_slice, export).expect("构建失败");
            },
            Commands::Redeploy {path} => {

            },
        }
    }
}

fn build(path: &str,port_list:&[&str], export: bool) -> Result<String, Error>{
    let file_data =file_utils::file_data::FileData::new(path.to_string()).unwrap();
    docker_utils::rebuild_container(&file_data.filename, port_list)?;
    if export {
        docker_utils::save(&file_data.filename)?;
    }
    Ok("".to_string())
}