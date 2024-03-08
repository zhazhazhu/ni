use console::style;
use indexmap::IndexMap;
use inquire::Confirm;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct Package {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub version: Option<String>,
    pub packageManager: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
    #[serde(rename = "scripts-info")]
    pub scripts_info: Option<IndexMap<String, String>>,
}

lazy_static! {
    pub static ref AGENT_MAP: IndexMap<&'static str, Agent> = {
        let mut m = IndexMap::new();
        m.insert("bun", Agent::Bun);
        m.insert("pnpm", Agent::Pnpm);
        m.insert("pnpm@6", Agent::Pnpm6);
        m.insert("yarn", Agent::Yarn);
        m.insert("yarn@berry", Agent::YarnBerry);
        m.insert("npm", Agent::Npm);
        m
    };
    pub static ref AGENT_INSTALL: IndexMap<Agent, &'static str> = {
        let mut m = IndexMap::new();
        m.insert(Agent::Bun, "https://bun.sh");
        m.insert(Agent::Pnpm, "https://pnpm.io/installation");
        m.insert(Agent::Pnpm6, "https://pnpm.io/6.x/installation");
        m.insert(Agent::Yarn, "https://classic.yarnpkg.com/en/docs/install");
        m.insert(
            Agent::YarnBerry,
            "https://yarnpkg.com/getting-started/install",
        );
        m.insert(
            Agent::Npm,
            "https://docs.npmjs.com/cli/v8/configuring-npm/install",
        );
        m
    };
    pub static ref LOCKS_MAP: IndexMap<&'static str, Agent> = {
        let mut m = IndexMap::new();
        m.insert("bun.lockb", Agent::Bun);
        m.insert("pnpm-lock.yaml", Agent::Pnpm);
        m.insert("yarn.lock", Agent::Yarn);
        m.insert("package-lock.json", Agent::Npm);
        m.insert("npm-shrinkwrap.json", Agent::Npm);
        m
    };
}

pub fn detect(options: DetectOptions) -> Option<Agent> {
    let mut agent: Option<Agent> = None;
    let mut version: Option<String> = None;

    let mut lock_path: Option<String> = None;
    for (lock, _) in LOCKS_MAP.iter() {
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
                            version = Some(ver.to_string());
                            let ver = ver
                                .split(".")
                                .map(String::from)
                                .collect::<Vec<String>>()
                                .first()
                                .map(String::from);

                            if let Some(ver) = &ver {
                                let ver = ver.parse::<i32>().unwrap();

                                if name.to_string() == "yarn" && ver > 1 {
                                    agent = Some(Agent::YarnBerry);
                                    version = Some("berry".into())
                                } else if name.to_string() == "pnpm" && ver < 7 {
                                    agent = Some(Agent::Pnpm6);
                                } else if AGENT_MAP.contains_key(name) {
                                    agent = AGENT_MAP.get(name).cloned();
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
                    agent = LOCKS_MAP.get(path).cloned();
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
                let install_link = style(AGENT_INSTALL.get(agent).unwrap())
                    .blue()
                    .underlined()
                    .to_string();
                let install_confirm_text =
                    format!("Would you like to globally install {}?", install_link);
                let confirmation = Confirm::new(&install_confirm_text)
                    .with_default(false)
                    .prompt()
                    .unwrap();

                if !confirmation {
                    process::exit(1)
                }
            }

            let mut args: Vec<String> = vec!["i".into(), "-g".into()];
            if let Some(v) = version.clone() {
                let agent = agent.as_str().split("@").collect::<Vec<&str>>()[0];
                let agent = format!("{agent}@{v}");
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
