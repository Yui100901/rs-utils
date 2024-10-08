use std::fs;
use std::io;
use std::path::PathBuf;

/// 遍历给定目录并返回文件路径列表
/// `recursive` 参数表明是否递归遍历子目录
pub fn traverse_dir_files(dir: &str, recursive: bool) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    if recursive {
        // 递归遍历目录
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
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
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }

    Ok((dirs, files))
}
