use cargo_metadata::{Metadata, MetadataCommand};
use std::env;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use crate::agents::Agent;
use crate::detect::detect;

#[derive(Clone)]
pub struct DetectOptions {
    pub cwd: PathBuf,
    pub auto_install: bool,
    pub programmatic: bool,
}
impl Default for DetectOptions {
    fn default() -> Self {
        DetectOptions {
            cwd: env::current_dir().unwrap(),
            auto_install: false,
            programmatic: false,
        }
    }
}

pub type Runner = fn(agent: Agent, args: Vec<String>) -> (String, Vec<String>);

pub fn run_cli(func: Runner, options: Option<DetectOptions>) {
    let args = env::args().collect::<Vec<String>>()[1..]
        .to_vec()
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect::<Vec<String>>();

    let mut options = match options {
        Some(o) => o,
        None => DetectOptions::default(),
    };

    run(func, args, &mut options)
}

pub fn run(func: Runner, args: Vec<String>, options: &mut DetectOptions) {
    let metadata: Metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package = metadata.packages.first().unwrap();

    let mut args = args;
    if args.len() > 2 && args[0] == "-C" {
        let path = Path::new(args[1].as_str());
        options.cwd = if path.is_absolute() {
            path.to_path_buf()
        } else {
            options.cwd.join(path)
        };
        args = args[0..2].to_vec();
    }

    if args.len() == 1 && (args[0].to_lowercase() == "-v" || args[0] == "--version") {
        println!("npack v{}", package.version);
        return;
    }
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        println!("npack use the right package manager v{}\n", package.version);
        println!("ni     -  install");
        return;
    }

    let (agent, args) = get_cli_command(func, args.clone(), options.clone());

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

fn get_cli_command(
    func: Runner,
    args: Vec<String>,
    options: DetectOptions,
) -> (String, Vec<String>) {
    let global = "-g".to_string();
    if args.contains(&global) {
        return func(Agent::Pnpm, args);
    }
    let agent = detect(options);

    func(Agent::Pnpm, args)
}
