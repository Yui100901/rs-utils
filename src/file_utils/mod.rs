use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use dotenv::Error;
use log::error;

/// 遍历给定目录并返回文件路径列表
/// `recursive` 参数表明是否递归遍历子目录
pub fn traverse_dir_files(dir: &str, recursive: bool) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    let dir_path = Path::new(dir).canonicalize()?;

    if recursive {
        // 递归遍历目录
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path().canonicalize()?;
            if path.is_dir() {
                dirs.push(path.clone());
                let (sub_dirs, sub_files) = traverse_dir_files(path.to_str().unwrap(), true)?;
                dirs.extend(sub_dirs);
                files.extend(sub_files);
            } else {
                files.push(path);
            }
        }
    } else {
        // 非递归遍历目录
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path().canonicalize()?;
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
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


