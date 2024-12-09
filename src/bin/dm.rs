use std::io::{BufRead, Error};
use std::path::Path;
use clap::{Parser, Subcommand};
use log::{error, info};
use rs_utils::{docker_utils, file_utils, log_utils};

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
    Recreate {
        path: String,
    },
    Import {
        path: String,
    },
    Export {},
}

fn main() {
    log_utils::init_logger();
    let cli = Cli::parse();
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Build {export,path} => {
                if !Path::new(&path).join("Dockerfile").exists() {
                    error!("Dockerfile does not exist");
                }
                build(&path, export).expect("构建失败");
            },
            Commands::Recreate {path} => {

            },
            Commands::Import {path} => {

            }
            Commands::Export {} => {
                export("images").expect("Export failed!");
            }
        }
    }
}

fn build(path: &str,export: bool) -> Result<String, Error>{
    let file_data =file_utils::file_data::FileData::new(path.to_string()).unwrap();
    docker_utils::build(&file_data.filename)?;
    if export {
        docker_utils::save(&file_data.filename,".")?;
    }
    Ok("".to_string())
}

fn export(path: &str) -> Result<String, Error> {
    docker_utils::prune()?;
    let images=docker_utils::list_images_formatted()?;
    let images: Vec<&str> = images.lines().filter(|line| !line.is_empty()).collect();
    for image in images{
        if let Err(e) = docker_utils::save(image, path) {
            error!("Failed to save image {}: {}", image, e);
        }
    }
    Ok("Export success!".to_string())
}