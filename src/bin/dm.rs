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
    Clean{
    },
    Import {
        path: Option<String>,
    },
    Export {
        path: Option<String>,
    },
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
            Commands::Clean {} =>{
                clean().expect("Failed to clean containers!");
            },
            Commands::Import {path} => {
                if let Some(path) = path {
                    import(&path).expect("Import failed!");
                }else {
                    import("images").expect("Import failed!");
                }
            }
            Commands::Export {path} => {
                if let Some(path) = path {
                    file_utils::create_directory(&path).expect("Create directory failed!");
                }else {
                    file_utils::create_directory("images").expect("Create directory failed!");
                }
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

fn clean()-> Result<String, Error> {
    docker_utils::prune()
}

fn import(path: &str) -> Result<String, Error> {
    match file_utils::file_data::FileData::new(path.to_string()){
        Ok(data) => {
            match file_utils::traverse_dir_files(data.path.as_str(),true){
                Ok((_,files)) => {
                    for file in files{
                        docker_utils::load(file.path.as_str())?;
                    }
                },
                Err(e)=>{
                    error!("Import {} failed!Error:{}", path,e);
                }
            }
        },
        Err(e)=>{
            error!("Import {} failed!Error:{}", path,e);
        }
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