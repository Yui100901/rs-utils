use std::io::{BufRead, Error};
use std::path::Path;
use clap::{Parser, Subcommand};
use log::{error, info};
use rs_utils::{docker_utils, file_utils};

#[derive(Parser,Debug)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Build {
        #[arg(short, long)]
        export: bool,

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
            Commands::Build {export,path} => {
                if !Path::new(&path).join("Dockerfile").exists() {
                    error!("Dockerfile does not exist");
                }
                build(&path, export).expect("构建失败");
            },
            Commands::Redeploy {path} => {

            },
        }
    }
}

fn build(path: &str,export: bool) -> Result<String, Error>{
    let file_data =file_utils::file_data::FileData::new(path.to_string()).unwrap();
    docker_utils::build(&file_data.filename)?;
    if export {
        docker_utils::save(&file_data.filename)?;
    }
    Ok("".to_string())
}