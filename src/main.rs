use std::io;

// src/main.rs
mod file_utils;

fn main()-> io::Result<()> {
    let dir = "."; // 当前目录
    let recursive = false; // 递归遍历

    match file_utils::file_utils::traverse_dir_files(dir, recursive) {
        Ok((dirs, files)) => {
            println!("Directories:");
            for dir in dirs {
                println!("{:?}", dir);
            }
            println!("Files:");
            for file in files {
                println!("{:?}", file);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
