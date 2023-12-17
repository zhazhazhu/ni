use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::agents::Agents;

pub struct DetectOptions {
    pub cwd: Option<String>,
}

pub type Runner = fn(agent: Agents) -> String;

pub fn run_cli(func: Runner) {
    let args = env::args().collect::<Vec<String>>()[1..]
        .to_vec()
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect::<Vec<String>>();

    run(func, args)
}

pub fn run(func: Runner, args: Vec<String>) {
    let mut args = args;
    println!("before args is {:?}", args);
    let cwd = env::current_dir().unwrap();
    let mut config_cwd = PathBuf::new();
    println!("cwd is {:?}", cwd);
    if args[0] == "-C" {
        let path = Path::new(args[1].as_str());
        config_cwd = if path.is_absolute() {
            path.to_path_buf()
        } else {
            cwd.join(path)
        };
        args = args[0..2].to_vec();
    }
    println!("args is {:?}", args);
    println!("config_cwd is {:?}", config_cwd);
    let command = func(Agents::Npm);
    let output = Command::new(command)
        .arg("install")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed:\n{}", stderr);
    }
}
