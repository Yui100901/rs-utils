use std::io::{self};
use crate::command_utils;

pub fn clone_latest(url: &str, branch: &str, dir: &str) -> Result<String, io::Error> {
    command_utils::run_command("git",
                               &["clone","--single-branch",
                                   "--branch",branch,
                                   "--depth","1",
                                   url,dir])
}
