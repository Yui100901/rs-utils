use std::error::Error;
use std::fs;
use std::path::PathBuf;
use log::error;

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
        let abs_path_buf = match fs::canonicalize(&path_buf) {
            Ok(pb)=>{
                pb
            },
            Err(e)=>{
                error!("Can't canonicalize path {}: {}", path_buf.display(), e);
                return Err(Box::new(e));
            }
        };
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