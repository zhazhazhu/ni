use std::process;

use console::style;
use dialoguer::FuzzySelect;
use npack::{
    parse::parse_nr,
    runner::run_cli,
    storage::{dump, load, STORAGE},
    utils::get_package_json,
};

#[derive(Debug, Clone)]
struct ScriptRaw {
    pub key: String,
    pub _cmd: String,
    pub description: String,
}

impl ToString for ScriptRaw {
    fn to_string(&self) -> String {
        let key = self.key.clone();
        let description = self.description.clone();
        let item = format!("{}    {}", style(key).cyan(), style(description).dim());
        item
    }
}

fn main() {
    run_cli(
        |agent, mut args, ctx| {
            load();

            if args.len() > 0 && args[0] == "-" {
                let storage_guard = STORAGE.lock();
                let storage = storage_guard.as_ref().unwrap();
                let storage = storage.clone();
                if storage.last_run_command.is_none() {
                    println!("{}", style("No last command found").red());
                    process::exit(1)
                }
                args[0] = storage.last_run_command.unwrap();
            }

            if args.len() == 0 {
                match ctx {
                    Some(ctx) => {
                        if !ctx.programmatic {
                            let path = ctx.cwd.join("package.json");
                            match path.to_str() {
                                Some(path) => {
                                    let storage_guard = STORAGE.lock();
                                    let storage = storage_guard.as_ref().unwrap();
                                    let pkg = get_package_json(path);
                                    let scripts = pkg.scripts.unwrap_or_default();
                                    let scripts_info = pkg.scripts_info.unwrap_or_default();
                                    let names = scripts
                                        .iter()
                                        .map(|(key, value)| [key, value])
                                        .collect::<Vec<[&String; 2]>>();
                                    let raw = names
                                        .iter()
                                        .filter(|x| !x[0].starts_with("?"))
                                        .map(|[key, value]| {
                                            let key = key.to_string();
                                            let cmd = value.to_string();
                                            let description = scripts_info
                                                .get(&key)
                                                .map_or_else(|| cmd.clone(), |v| v.to_string());
                                            ScriptRaw {
                                                key: key,
                                                _cmd: cmd,
                                                description,
                                            }
                                        })
                                        .collect::<Vec<ScriptRaw>>();

                                    if let Some(command) = &storage.last_run_command {
                                        let last = raw.iter().find(|x| command == &x.key);
                                        match last {
                                            Some(_) => {
                                                // raw.insert(0, last.clone())
                                            }
                                            None => {}
                                        };
                                    }

                                    let select = FuzzySelect::new()
                                        .with_prompt("script to run:")
                                        .default(0)
                                        .items(&raw)
                                        .interact()
                                        .unwrap();

                                    args.push(raw[select].key.to_string());
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            }
            let storage_guard = STORAGE.lock();
            let mut storage = storage_guard.as_ref().unwrap().clone();
            match storage.last_run_command.clone() {
                Some(command) => {
                    if command != args[0] {
                        storage.last_run_command = Some(args[0].to_string());
                        dump(&storage).unwrap();
                    }
                }
                None => {
                    println!("hi");
                    storage.last_run_command = Some(args[0].to_string());
                    dump(&storage).unwrap();
                }
            };
            drop(storage_guard);

            let mut storage_guard = STORAGE.lock();
            *storage_guard = Some(storage);

            parse_nr(agent, args)
        },
        None,
    )
}
