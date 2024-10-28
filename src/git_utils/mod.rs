use crate::command_utils;
use std::env::args;
use std::io::{self};

pub fn clone_default(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    let args = &["clone", "--branch", branch, url, dir];
    command_utils::run_command("git", args)
}

pub fn clone_single_branch(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    let args = &["clone", "--single-branch", "--branch", branch, url, dir];
    command_utils::run_command("git", args)
}

pub fn clone_latest(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    let args = &[
        "clone",
        "--single-branch",
        "--branch",
        branch,
        "--depth",
        "1",
        url,
        dir,
    ];
    command_utils::run_command("git", args)
}

pub fn pull() -> Result<String, io::Error> {
    let args = &["pull"];
    command_utils::run_command("git", args)
}
