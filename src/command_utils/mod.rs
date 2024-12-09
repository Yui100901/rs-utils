use log::{error, info, warn};
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::{io, thread};

/// 处理输出流的通用函数
fn handle_output<T: BufRead>(reader: T, log:&str) -> String {
    let mut output = String::new();
    for line in reader.split(b'\n') {
        match line {
            Ok(bytes) => {
                let line = String::from_utf8_lossy(&bytes);
                match log {
                    "info" => {
                        info!("{}", line);
                    },
                    "warn" => {
                        warn!("{}", line);
                    }
                    "error" => {
                        error!("{}", line);
                    }
                    _ => {}
                }
                output.push_str(&line);
                output.push('\n');
            }
            Err(e) => error!("Failed to read line: {}", e),
        }
    }
    output
}

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
        handle_output(reader, "info")
    });

    // 创建线程来处理标准错误
    let stderr_handle = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        handle_output(reader, "warn")
    });

    // 等待命令执行完毕
    cmd.wait().expect("Failed to wait on child");

    // 获取标准输出和标准错误
    let stdout = stdout_handle.join().expect("The stdout thread has panicked");
    let stderr = stderr_handle.join().expect("The stderr thread has panicked");

    Ok(stdout)
}

