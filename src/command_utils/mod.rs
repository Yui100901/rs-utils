use std::process::{Command, Output};
use std::io;
use log::{error, info};

pub fn run_command(name: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new(name)
        .args(args)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}
