use console::style;
use inquire::Select;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::{env, io};

use crate::agents::Agent;
use crate::config::{get_default_agent, get_global_agent, DefaultAgent};
use crate::detect::{detect, AGENT_MAP};

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
impl DetectOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_auto_install(mut self, auto_install: bool) -> Self {
        self.auto_install = auto_install;
        self
    }
}

pub struct RunnerContext {
    pub programmatic: bool,
    pub has_lock: bool,
    pub cwd: PathBuf,
}

pub type Runner =
    fn(agent: Agent, args: Vec<String>, ctx: Option<RunnerContext>) -> (String, Vec<String>);

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
    let version = env!("CARGO_PKG_VERSION");

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
        println!("npack   {}", style(format!("v{}", version)).blue());
        return;
    }
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        println!("npack use the right package manager v{}\n", version);
        println!("ni    -   install");
        println!("nr    -   run");
        println!("nlx   -   execute");
        println!("nu    -   upgrade");
        println!("nun   -   uninstall");
        println!("nci   -   uninstall");
        println!("na    -   agent alias");
        println!("ni -v -   show used agent");
        println!(
            "{}",
            style("\ncheck https://github.com/zhazhazhu/ni for more documentation.").yellow()
        );
        return;
    }

    let command = get_cli_command(func, args.clone(), options.clone());

    if let Some((agent, args)) = command {
        execa_command(&agent, Some(args)).unwrap()
    } else {
        return;
    }
}

fn get_cli_command(
    func: Runner,
    args: Vec<String>,
    options: DetectOptions,
) -> Option<(String, Vec<String>)> {
    let global = "-g".to_string();
    if args.contains(&global) {
        return Some(func(get_global_agent(), args, None));
    }
    #[allow(unused_assignments)]
    let mut agent = DefaultAgent::Prompt;
    if let Some(v) = detect(options.clone()) {
        agent = DefaultAgent::Agent(v);
    } else {
        agent = get_default_agent(options.clone().programmatic);
    }

    if agent == DefaultAgent::Prompt {
        let items: Vec<&&str> = AGENT_MAP.keys().filter(|x| !x.contains("@")).collect();
        let selection = Select::new("script to run:", items).prompt();
        if let Ok(selection) = selection {
            let value = AGENT_MAP.get(selection);
            if let Some(value) = value {
                agent = DefaultAgent::Agent(value.clone());
            } else {
                return None;
            }
        } else {
            process::exit(1)
        }
    }
    let runner_ctx = RunnerContext {
        programmatic: options.programmatic,
        has_lock: true,
        cwd: options.cwd,
    };
    match agent {
        DefaultAgent::Agent(agent) => Some(func(agent, args, Some(runner_ctx))),
        DefaultAgent::Prompt => Some(func(Agent::Npm, args, Some(runner_ctx))),
    }
}

pub fn execa_command(agent: &str, args: Option<Vec<String>>) -> Result<(), io::Error> {
    let mut command = Command::new(agent)
        .args(args.unwrap_or_default())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if let Some(stdout) = command.stdout.take() {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    let status = command.wait()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }

    Ok(())
}
