use console::style;
use dialoguer::Confirm;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process,
};

use crate::{
    agents::Agent,
    runner::{execa_command, DetectOptions},
    utils::which_cmd,
};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Package {
    name: String,
    r#type: String,
    version: String,
    packageManager: Option<String>,
}

pub fn detect(options: DetectOptions) -> Option<Agent> {
    let mut agent: Option<Agent> = None;
    let mut version: Option<String> = None;

    let agent_map: HashMap<&str, Agent> = vec![
        ("bun", Agent::Bun),
        ("pnpm", Agent::Pnpm),
        ("pnpm@6", Agent::Pnpm6),
        ("yarn", Agent::Yarn),
        ("yarn@berry", Agent::YarnBerry),
        ("npm", Agent::Npm),
    ]
    .iter()
    .cloned()
    .collect();
    let agent_install: HashMap<Agent, &str> = vec![
        (Agent::Bun, "https://bun.sh"),
        (Agent::Pnpm, "https://pnpm.io/installation"),
        (Agent::Pnpm6, "https://pnpm.io/6.x/installation"),
        (Agent::Yarn, "https://classic.yarnpkg.com/en/docs/install"),
        (
            Agent::YarnBerry,
            "https://yarnpkg.com/getting-started/install",
        ),
        (
            Agent::Npm,
            "https://docs.npmjs.com/cli/v8/configuring-npm/install",
        ),
    ]
    .iter()
    .cloned()
    .collect();
    let locks_map: HashMap<&str, Agent> = vec![
        ("bun.lockb", Agent::Bun),
        ("pnpm-lock.yaml", Agent::Pnpm),
        ("yarn.lock", Agent::Yarn),
        ("package-lock.json", Agent::Npm),
        ("npm-shrinkwrap.json", Agent::Npm),
    ]
    .iter()
    .cloned()
    .collect();

    let mut lock_path: Option<String> = None;
    for (lock, _) in locks_map {
        let path = find_up(lock, &options.cwd);
        if let Some(path) = path {
            lock_path = Some(path);
            break;
        }
    }
    let package_json_path = if let Some(path) = &lock_path {
        let lock_path = Path::new(path)
            .parent()
            .map(|parent| parent.join("package.json").to_str().map(String::from))
            .unwrap_or(None);
        lock_path
    } else {
        find_up("package.json", &options.cwd)
    };

    if let Some(package_json_path) = package_json_path {
        let path = Path::new(&package_json_path);
        if path.exists() && path.is_file() {
            let file = File::open(&path);
            if let Ok(mut file) = file {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    let p = serde_json::from_str::<Package>(&contents).unwrap();
                    #[allow(non_snake_case)]
                    if let Some(packageManager) = p.packageManager {
                        let parts = if packageManager.starts_with('^') {
                            String::from(&packageManager[1..])
                        } else {
                            String::from(&packageManager)
                        };
                        let parts = parts.split('@').collect::<Vec<&str>>();
                        if let [name, ver] = parts.as_slice() {
                            version = ver
                                .split(".")
                                .map(String::from)
                                .collect::<Vec<String>>()
                                .first()
                                .map(String::from);

                            if let Some(ver) = &version {
                                let ver = ver.parse::<i32>().unwrap();

                                if name.to_string() == "yarn" && ver > 1 {
                                    agent = Some(Agent::YarnBerry);
                                    version = Some("berry".into())
                                } else if name.to_string() == "pnpm" && ver < 7 {
                                    agent = Some(Agent::Pnpm6);
                                } else if agent_map.contains_key(name) {
                                    agent = agent_map.get(name).cloned();
                                    //TODO plan use HashMap
                                } else if !options.programmatic {
                                    println!("[ni] Unknown packageManager: {}", &packageManager);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(lock_path) = lock_path {
        if agent.is_none() {
            if let Some(file_name) = Path::new(&lock_path).file_name() {
                if let Some(path) = file_name.to_str() {
                    agent = agent_map.get(path).cloned();
                }
            }
        }
    }

    if let Some(agent) = &agent {
        let cmd = which_cmd(&agent.as_str());
        if cmd == false && options.programmatic == false {
            if options.auto_install == false {
                println!(
                    "{}",
                    style(format!(
                        "[ni] Detected {} but it doesn't seem to be installed.",
                        &agent.as_str()
                    ))
                    .yellow()
                );

                if env::var("CI").is_ok() {
                    process::exit(1)
                }

                let link = style(format!("{}", agent_install.get(&agent).unwrap()))
                    .blue()
                    .underlined()
                    .to_string();
                let confirmation = Confirm::new()
                    .with_prompt(format!("Would you like to globally install {}?", link))
                    .interact()
                    .unwrap();

                if !confirmation {
                    process::exit(1)
                }
            }

            let mut args: Vec<String> = vec!["i".into(), "-g".into()];
            if let Some(v) = version.clone() {
                let agent = format!("{}@{}", agent.as_str(), v);
                args.push(agent);
            } else {
                let agent = format!("{}", agent.as_str());
                args.push(agent);
            }
            execa_command("npm", Some(args)).unwrap()
        }
    }

    return agent;
}

pub fn find_up(filename: &str, cwd: &PathBuf) -> Option<String> {
    let mut cwd = cwd.clone();
    loop {
        let file_path = cwd.join(filename);
        if file_path.is_file() {
            return Some(file_path.to_string_lossy().into());
        }
        if !cwd.pop() {
            break;
        }
    }
    None
}
