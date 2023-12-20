use std::env;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::agents::Agent;
use crate::detect::detect;

pub struct DetectOptions {
    pub cwd: Option<String>,
}

pub type Runner = fn(agent: Agent, args: Vec<String>) -> (String, Vec<String>);

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
    if args.len() > 2 && args[0] == "-C" {
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

    let (agent, args) = get_cli_command(func, args.clone());

    let mut command = Command::new(&agent)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    let status = command.wait().expect("Failed to wait for command");
    if !status.success() {
        println!("Command execution failed");
    }
}

fn get_cli_command(func: Runner, args: Vec<String>) -> (String, Vec<String>) {
    let global = "-g".to_string();
    if args.contains(&global) {
        return func(Agent::Pnpm, args);
    }
    func(Agent::Pnpm, args)
}
