pub mod file_data;

use crate::file_utils::file_data::FileData;
use log::{error, info};
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 遍历给定目录并返回文件路径列表
/// `recursive` 参数表明是否递归遍历子目录
pub fn traverse_dir_files(
    dir: &str,
    recursive: bool,
) -> io::Result<(Vec<FileData>, Vec<FileData>)> {
    let mut files = Vec::new();
    let mut dirs: Vec<FileData> = Vec::new();

    let dir_file_data;

    match FileData::new(dir.to_string()) {
        Ok(d) => dir_file_data = d,
        Err(e) => {
            error!("Failed to open file:{}", e);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to open directory",
            ));
        }
    }

    //遍历目录
    for entry in fs::read_dir(&dir_file_data.path_buf)? {
        let entry = entry?;
        let path = match entry.path().canonicalize() {
            Ok(p) => p,
            Err(_) => continue,
        };
        if let Some(path_str) = path.to_str() {
            match FileData::new(path_str.to_string()) {
                Ok(data) => {
                    if data.path_buf.is_dir() {
                        dirs.push(data);
                        if recursive {
                            match traverse_dir_files(path.to_str().unwrap(), true) {
                                Ok((sub_dirs, sub_files)) => {
                                    dirs.extend(sub_dirs);
                                    files.extend(sub_files);
                                }
                                Err(e) => {
                                    error!("Failed to traverse subdirectory: {}", e);
                                    continue;
                                }
                            }
                        }
                    } else {
                        files.push(data);
                    }
                }
                Err(e) => {
                    error!("Failed to get file data:{}", e);
                }
            }
        }
    }
    Ok((dirs, files))
}

/// 替换源文件到目标文件
pub fn replace(source: &Path, target: &Path) -> Result<String, io::Error> {
    match fs::copy(source, target) {
        Ok(_) => Ok(String::from("文件替换成功！")),
        Err(e) => Err(e),
    }
}

/// 创建文件夹
pub fn create_directory(path: &str) -> Result<FileData, Box<dyn Error>> {
    let images_dir = Path::new(path);
    if !images_dir.exists() {
        fs::create_dir(images_dir)?;
    }
    FileData::new(String::from(path))
}
