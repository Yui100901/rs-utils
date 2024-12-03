use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

pub struct FileData{
    pub path: String,
    pub path_buf: PathBuf,
    pub abs_path: String,
    pub filename: String,
    pub metadata: fs::Metadata,
}

impl FileData {
    pub fn new(path: String) -> Result<Self, Box<dyn Error>> {
        // 获取绝对路径
        let path_buf = PathBuf::from(&path);
        let abs_path_buf = fs::canonicalize(&path_buf)?;
        let abs_path = abs_path_buf.to_str().ok_or("无法转换为字符串")?.to_string();

        // 获取文件名
        let filename = abs_path_buf
            .file_name()
            .ok_or("无法获取文件名")?
            .to_str()
            .ok_or("文件名包含无效 UTF-8")?
            .to_string();

        // 获取文件元数据
        let metadata = fs::metadata(&abs_path)?;

        Ok(FileData {
            path,
            path_buf,
            abs_path,
            filename,
            metadata,
        })
    }
}

/// 替换源文件到目标文件
pub fn replace(source: &Path, target: &Path) -> Result<String, io::Error> {
    match fs::copy(source, target) {
        Ok(_) => Ok(String::from("文件替换成功！")),
        Err(e) => Err(e),
    }
}

/// 获取目录的文件名
pub fn dir_filename(path: &str) -> Result<String, Box<dyn Error>> {
    let abs_path = fs::canonicalize(Path::new(path))?;
    let file_name = abs_path
        .file_name()
        .ok_or("无法获取文件名")?
        .to_str()
        .ok_or("文件名包含无效 UTF-8")?
        .to_string();

    Ok(file_name)
}

pub fn get_absolute_path(path: &str) -> Result<String, Box<dyn Error>> {
    let abs_path = fs::canonicalize(Path::new(path))?;
    Ok(abs_path.to_str().ok_or("无法转换为字符串")?.to_string())
}