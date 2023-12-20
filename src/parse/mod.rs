use std::process;

use crate::{
    agents::{Agent, AgentCommand, COMMAND},
    utils::exclude,
};

const GLOBAL: &str = "-g";
const FROZEN: &str = "--frozen";

pub type CommandTuple = (String, Vec<String>);

pub fn parse_ni(agent: Agent, args: Vec<String>) -> CommandTuple {
    let mut args = args;
    if agent == Agent::Bun {
        args = args
            .iter()
            .map(|i| {
                if i == "-D" {
                    "-d".to_string()
                } else {
                    i.to_string()
                }
            })
            .collect::<Vec<String>>();
    };
    if args.contains(&GLOBAL.into()) {
        return get_command(&agent, AgentCommand::Global, exclude(&args, GLOBAL.into()));
    }
    if args.contains(&FROZEN.into()) {
        return get_command(&agent, AgentCommand::Frozen, exclude(&args, FROZEN.into()));
    }
    if args.len() == 0 || args.iter().all(|item| item.starts_with("-")) {
        return get_command(&agent, AgentCommand::Install, args.clone());
    }

    return get_command(&agent, AgentCommand::Add, args.clone());
}

fn get_command(agent: &Agent, command: AgentCommand, args: Vec<String>) -> CommandTuple {
    let agent_command = match agent {
        Agent::Npm => COMMAND.npm,
        Agent::Yarn => COMMAND.yarn,
        Agent::YarnBerry => COMMAND.yarn_berry,
        Agent::Pnpm => COMMAND.pnpm,
        Agent::Pnpm6 => COMMAND.pnpm6,
        Agent::Bun => COMMAND.bun,
    };

    let c = match command {
        AgentCommand::Agent => agent_command.agent,
        AgentCommand::Run => agent_command.run,
        AgentCommand::Install => agent_command.install,
        AgentCommand::Frozen => agent_command.frozen,
        AgentCommand::Global => agent_command.global,
        AgentCommand::Add => agent_command.add,
        AgentCommand::Upgrade => agent_command.upgrade,
        AgentCommand::UpgradeInteractive => agent_command.upgrade_interactive,
        AgentCommand::Execute => agent_command.execute,
        AgentCommand::Uninstall => agent_command.uninstall,
        AgentCommand::GlobalUninstall => agent_command.global_uninstall,
    };

    if c.is_empty() {
        process::exit(1)
    }

    let c: Vec<String> = (c.replace("{0}", &args.join(" ")))
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect();

    (c[0].clone(), c[1..].to_vec())
}
