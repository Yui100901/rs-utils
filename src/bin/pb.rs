use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use clap::Parser;
use log::info;
use rs_utils::{file_utils, log_utils};
use rs_utils::build_utils::project::Project;

/// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(version,author="Yui100901", about="简易的项目构建工具", long_about = None)]
struct Args {
    /// 是否并发构建
    #[arg(short, long, help = "并发构建")]
    concurrent: bool,

    /// 是否自动部署
    #[arg(short, long, help = "自动部署")]
    deploy: bool,

    /// 端口列表
    #[arg(short, long, help = "端口列表")]
    ports: Option<String>,

    /// 输入的文件或目录路径
    #[arg(help = "文件路径或项目目录路径")]
    path: Option<String>,
}

fn main() {
    log_utils::init_logger();
    let args = Args::parse();
    let concurrent_build = args.concurrent;
    let deploy = args.deploy;
    let ports = args.ports.unwrap_or("".to_string());
    let path = args.path.unwrap_or(".".to_string());

    let port_list: Vec<String> = ports.split(',').map(|s| s.to_string()).collect();
    info!("输入路径: {}", &path);
    info!("是否并发构建: {}", concurrent_build);
    info!("端口列表: {:?}", port_list);

    let file_data = file_utils::file_data::FileData::new(path.clone()).unwrap();

    let mut project_list: Vec<Project> = Vec::new();

    if file_data.metadata.is_dir() {
        let b = Project::new(
            file_data.abs_path.trim_start_matches(r"\\?\").to_string(),
            file_data.filename,
            port_list,
            "".to_string(),
            "".to_string(),
        );
        project_list.push(b);
    } else {
        let work_dir = fs::canonicalize(Path::new("."))
            .expect("获取当前目录出错")
            .join("projects");

        if !work_dir.exists() {
            fs::create_dir_all(&work_dir).expect("创建 projects 文件夹出错");
        }

        let mut file = fs::File::open(&file_data.abs_path).expect("打开文件出错");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("读取文件出错");

        let result: HashMap<String, Project> = serde_yaml::from_str(&data).expect("解析 YAML 出错");

        for (key, b) in result {
            info!("Key: {}, Parsed Struct: {:?}", key, b);
            let project_dir = work_dir.join(&b.name);
            let project_dir_cleaned = project_dir.to_str().unwrap().trim_start_matches(r"\\?\");
            let b1 = Project::new(
                String::from(project_dir_cleaned),
                b.name,
                b.ports.clone(),
                b.repository.url.clone(),
                b.repository.branch.clone(),
            );
            project_list.push(b1);
        }
    }

    // let builder_list = Arc::new(Mutex::new(builder_list));

    // if concurrent_build {
    //     let mut handles = vec![];
    //
    //     for _ in 0..builder_list.lock().unwrap().len() {
    //         let b_clone = Arc::clone(&builder_list);
    //         let handle = thread::spawn(move || {
    //             let mut b = b_clone.lock().unwrap();
    //             b.iter_mut().for_each(|builder| {
    //                 builder.get_source_code();
    //                 builder.build();
    //             });
    //         });
    //         handles.push(handle);
    //     }
    //
    //     for handle in handles {
    //         handle.join().expect("线程执行失败");
    //     }
    // } else {
    info!("顺序构建");
    let mut projects = &mut project_list;
    projects.iter_mut().for_each(|project| {
        project.get_source_code();
        project.init_builder();
        project.build();
        if deploy {
            project.deploy_to_docker();
        }
    });
    // }

    for b in project_list.iter() {
        info!("{}", b.build_message);
    }
}
