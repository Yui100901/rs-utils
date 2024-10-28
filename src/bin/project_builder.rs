use clap::{App, Arg};
use rs_utils::build_utils::builder::Builder;
use serde_yaml;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

fn main() {
    let matches = App::new("Project Builder")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Builds projects based on provided configurations")
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("强制克隆仓库")
                .takes_value(false),
        )
        .arg(
            Arg::new("concurrent")
                .short('c')
                .long("concurrent")
                .help("是否并发构建")
                .takes_value(false),
        )
        .arg(
            Arg::new("ports")
                .short('p')
                .long("ports")
                .help("端口列表")
                .takes_value(true),
        )
        .arg(
            Arg::new("path")
                .help("输入的文件或目录路径")
                .required(true)
                .index(1),
        )
        .get_matches();

    let force_clone = matches.is_present("force");
    let concurrent_build = matches.is_present("concurrent");
    let ports = matches.value_of("ports").unwrap_or("");
    let path = matches.value_of("path").unwrap();

    let port_list: Vec<String> = ports.split(',').map(|s| s.to_string()).collect();
    println!("输入路径: {}", path);
    println!("强制克隆仓库: {}", force_clone);
    println!("是否并发构建: {}", concurrent_build);
    println!("端口列表: {:?}", port_list);

    let abs_input_path = fs::canonicalize(Path::new(path)).expect("无法访问输入路径");

    let mut builder_list: Vec<Builder> = Vec::new();
    let metadata = fs::metadata(&abs_input_path).expect("无法访问输入路径");

    if metadata.is_dir() {
        let name = abs_input_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let b = Builder::new(
            abs_input_path.to_str().unwrap().to_string(),
            name,
            port_list,
            "".to_string(),
            "".to_string(),
        );
        builder_list.push(b);
    } else {
        let work_dir = fs::canonicalize(Path::new("."))
            .expect("获取当前目录出错")
            .join("projects");

        if !work_dir.exists() {
            fs::create_dir_all(&work_dir).expect("创建 projects 文件夹出错");
        }

        let mut file = fs::File::open(&abs_input_path).expect("打开文件出错");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("读取文件出错");

        let result: HashMap<String, Builder> = serde_yaml::from_str(&data).expect("解析 YAML 出错");

        for (key, b) in result {
            println!("Key: {}, Parsed Struct: {:?}", key, b);
            let project_dir = work_dir.join(&b.name);
            let b1 = Builder::new(
                project_dir.to_str().unwrap().to_string(),
                b.name,
                b.ports.clone(),
                b.repository.url.clone(),
                b.repository.branch.clone(),
            );
            builder_list.push(b1);
        }
    }

    let builder_list = Arc::new(Mutex::new(builder_list));

    if concurrent_build {
        let mut handles = vec![];

        for _ in builder_list.lock().unwrap().iter() {
            let b_clone = Arc::clone(&builder_list);
            let handle = thread::spawn(move || {
                let mut b = b_clone.lock().unwrap();
                b.iter_mut().for_each(|builder| {
                    builder.build();
                });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("线程执行失败");
        }
    } else {
        let mut builders = builder_list.lock().unwrap();
        builders.iter_mut().for_each(|builder| {
            builder.build();
        });
    }

    for b in builder_list.lock().unwrap().iter() {
        println!("{}", b.build_message);
    }
}
