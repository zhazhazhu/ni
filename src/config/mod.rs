use std::env;

use crate::{agents::Agent, detect::detect, runner::DetectOptions};

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum DefaultAgent {
    Prompt,
    Agent(Agent),
}

pub struct Config {
    default_agent: DefaultAgent,
    global_agent: Agent,
}

pub fn get_config() -> Config {
    let mut config = Config {
        default_agent: DefaultAgent::Prompt,
        global_agent: Agent::Npm,
    };
    let mut options = DetectOptions::default();
    options.programmatic = true;
    let agent = detect(options);
    if let Some(agent) = agent {
        config.default_agent = DefaultAgent::Agent(agent);
    }
    config
}

pub fn get_default_agent(programmatic: bool) -> DefaultAgent {
    let Config { default_agent, .. } = get_config();
    let ci = env::var("CI");

    if default_agent == DefaultAgent::Prompt && (programmatic == true || ci.is_ok()) {
        return DefaultAgent::Agent(Agent::Npm);
    }
    default_agent
}

pub fn get_global_agent() -> Agent {
    let Config { global_agent, .. } = get_config();
    global_agent
}
