use crate::command_utils;
use std::io::{self};

pub fn clone_default(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    command_utils::run_command("git", &["clone", "--branch", branch, url, dir])
}

pub fn clone_single_branch(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    command_utils::run_command(
        "git",
        &["clone", "--single-branch", "--branch", branch, url, dir],
    )
}

pub fn clone_latest(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    command_utils::run_command(
        "git",
        &[
            "clone",
            "--single-branch",
            "--branch",
            branch,
            "--depth",
            "1",
            url,
            dir,
        ],
    )
}
