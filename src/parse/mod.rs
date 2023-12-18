use crate::{
    agents::{Agent, AgentCommand, COMMAND},
    utils::exclude,
};

pub fn parse_ni(agent: Agent, args: Vec<String>) -> String {
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
    let mut command = String::new();

    let global = "-g".into();
    if args.contains(&global) {
        command = get_command(&agent, AgentCommand::Global, exclude(&args, "-g".into()));
    }
    command = get_command(&agent, AgentCommand::Add, args);

    command
}

fn get_command(agent: &Agent, command: AgentCommand, args: Vec<String>) -> String {
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

    c.to_string()
}
