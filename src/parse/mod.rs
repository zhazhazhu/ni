use std::process;

use crate::{
    agents::{Agent, AgentCommand, COMMAND},
    runner::RunnerContext,
    utils::exclude,
};

const GLOBAL: &str = "-g";
const FROZEN: &str = "--frozen";
const IF_PRESENT: &str = "--if-present";
const FROZEN_IF_PRESENT: &str = "--frozen-if-present";

pub type CommandTuple = (String, Vec<String>);

pub fn parse_ni(agent: Agent, args: Vec<String>, ctx: Option<RunnerContext>) -> CommandTuple {
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
    if args.contains(&FROZEN_IF_PRESENT.into()) {
        if let Some(ctx) = ctx {
            if ctx.has_lock == true {
                return get_command(&agent, AgentCommand::Frozen, exclude(&args, FROZEN.into()));
            }
        }
        return get_command(&agent, AgentCommand::Install, exclude(&args, FROZEN.into()));
    }
    if args.contains(&FROZEN.into()) {
        return get_command(&agent, AgentCommand::Install, exclude(&args, FROZEN.into()));
    }
    if args.len() == 0 || args.iter().all(|item| item.starts_with("-")) {
        return get_command(&agent, AgentCommand::Install, args.clone());
    }

    return get_command(&agent, AgentCommand::Add, args.clone());
}

pub fn parse_nr(agent: Agent, mut args: Vec<String>) -> CommandTuple {
    if args.len() == 0 {
        args.push("start".into())
    }
    if !args.is_empty() && args.contains(&IF_PRESENT.into()) {
        args[0] = format!("--if-present {}", args[0]);
        return get_command(&agent, AgentCommand::Run, exclude(&args, IF_PRESENT.into()));
    }
    if !args.is_empty() {
        if args.len() > 1 {}
    }

    return get_command(&agent, AgentCommand::Run, args);
}

pub fn parse_nun(agent: Agent, args: Vec<String>, _: Option<RunnerContext>) -> CommandTuple {
    if !args.is_empty() && args.contains(&GLOBAL.into()) {
        return get_command(
            &agent,
            AgentCommand::GlobalUninstall,
            exclude(&args, GLOBAL.into()),
        );
    }
    return get_command(&agent, AgentCommand::Uninstall, args);
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

    let mut result = Vec::new();
    if c.contains("{0}") {
        result = (c.replace("{0}", &args.join(" ")))
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();
    } else if c.contains("{1}") {
        if args.len() > 1 {
            let r = format!("{} -- {}", &args[0], &args[1..].join(" "));
            result = (c.replace("{1}", &r))
                .trim()
                .split_whitespace()
                .map(String::from)
                .collect();
        } else {
            result = (c.replace("{1}", &args[0]))
                .trim()
                .split_whitespace()
                .map(String::from)
                .collect();
        }
    }

    (result[0].clone(), result[1..].to_vec())
}
