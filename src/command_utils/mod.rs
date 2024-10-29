use std::{io, thread};
use std::io::BufRead;
use std::process::{Command, Stdio};
use log::{error, info};

pub fn run_command(name: &str, args: &[&str]) -> Result<String, io::Error> {
    info!("Running command: {} {}", name, args.join(" "));
    let mut cmd = Command::new(name)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    // 获取标准输出和标准错误的句柄
    let stdout = cmd.stdout.take().expect("Failed to open stdout");
    let stderr = cmd.stderr.take().expect("Failed to open stderr");

    // 创建线程来处理标准输出
    let stdout_handle = thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            info!("{}", line.expect("Failed to read line"));
        }
    });

    // 创建线程来处理标准错误
    let stderr_handle = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            error!("{}", line.expect("Failed to read line"));
        }
    });

    // 等待命令执行完毕
    cmd.wait().expect("Failed to wait on child");

    // 等待所有线程完成
    stdout_handle.join().expect("The stdout thread has panicked");
    stderr_handle.join().expect("The stderr thread has panicked");
    Ok(String::from("Success"))
}
