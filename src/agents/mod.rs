#[derive(PartialEq, Clone, Debug)]
pub enum Agent {
    Npm,
    Yarn,
    YarnBerry,
    Pnpm,
    Pnpm6,
    Bun,
}
impl Agent {
    pub fn as_str(&self) -> &str {
        match self {
            Agent::Npm => "npm",
            Agent::Yarn => "yarn",
            Agent::YarnBerry => "yarn@berry",
            Agent::Pnpm => "pnpm",
            Agent::Pnpm6 => "pnpm@6",
            Agent::Bun => "bun",
        }
    }
}

pub struct Agents {
    pub npm: AgentCommands,
    pub yarn: AgentCommands,
    pub yarn_berry: AgentCommands,
    pub pnpm: AgentCommands,
    pub pnpm6: AgentCommands,
    pub bun: AgentCommands,
}

#[derive(PartialEq)]
pub enum AgentCommand {
    Agent,
    Run,
    Install,
    Frozen,
    Global,
    Add,
    Upgrade,
    UpgradeInteractive,
    Execute,
    Uninstall,
    GlobalUninstall,
}

pub struct AgentCommands {
    pub agent: &'static str,
    pub run: &'static str,
    pub install: &'static str,
    pub frozen: &'static str,
    pub global: &'static str,
    pub add: &'static str,
    pub upgrade: &'static str,
    pub upgrade_interactive: &'static str,
    pub execute: &'static str,
    pub uninstall: &'static str,
    pub global_uninstall: &'static str,
}

pub const COMMAND: Agents = Agents {
    npm: NPM_COMMAND,
    yarn: YARN_COMMAND,
    yarn_berry: YARN_BERRY_COMMAND,
    pnpm: PNPM_COMMAND,
    pnpm6: PNPM6_COMMAND,
    bun: BUN_COMMAND,
};

pub const NPM_COMMAND: AgentCommands = AgentCommands {
    agent: "npm {0}",
    run: "",
    install: "npm i {0}",
    frozen: "npm ci",
    global: "npm i -g {0}",
    add: "npm i {0}",
    upgrade: "npm update {0}",
    upgrade_interactive: "",
    execute: "npx {0}",
    uninstall: "npm uninstall {0}",
    global_uninstall: "npm uninstall -g {0}",
};

pub const YARN_COMMAND: AgentCommands = AgentCommands {
    agent: "yarn {0}",
    run: "yarn run {0}", //TODO
    install: "yarn install {0}",
    frozen: "yarn install --frozen-lockfile",
    global: "yarn global add {0}",
    add: "yarn add {0}",
    upgrade: "yarn upgrade {0}",
    upgrade_interactive: "yarn upgrade-interactive {0}",
    execute: "npx {0}",
    uninstall: "yarn remove {0}",
    global_uninstall: "yarn global remove {0}",
};

pub const YARN_BERRY_COMMAND: AgentCommands = AgentCommands {
    agent: "yarn {0}",
    run: "yarn run {0}",
    install: "yarn install {0}",
    frozen: "yarn install --immutable",
    global: "npm i -g {0}",
    add: "yarn add {0}",
    upgrade: "yarn up {0}",
    upgrade_interactive: "yarn up -i {0}",
    execute: "yarn dlx {0}",
    uninstall: "yarn remove {0}",
    global_uninstall: "npm uninstall -g {0}",
};

pub const PNPM_COMMAND: AgentCommands = AgentCommands {
    agent: "pnpm {0}",
    run: "pnpm run {0}",
    install: "pnpm i {0}",
    frozen: "pnpm i --frozen-lockfile",
    global: "pnpm add -g {0}",
    add: "pnpm add {0}",
    upgrade: "pnpm update {0}",
    upgrade_interactive: "pnpm update -i {0}",
    execute: "pnpm dlx {0}",
    uninstall: "pnpm remove {0}",
    global_uninstall: "pnpm remove --global {0}",
};

pub const PNPM6_COMMAND: AgentCommands = AgentCommands {
    agent: "pnpm {0}",
    run: "pnpm run {0}", //TODO
    install: "pnpm i {0}",
    frozen: "pnpm i --frozen-lockfile",
    global: "pnpm add -g {0}",
    add: "pnpm add {0}",
    upgrade: "pnpm update {0}",
    upgrade_interactive: "pnpm update -i {0}",
    execute: "pnpm dlx {0}",
    uninstall: "pnpm remove {0}",
    global_uninstall: "pnpm remove --global {0}",
};

pub const BUN_COMMAND: AgentCommands = AgentCommands {
    agent: "bun {0}",
    run: "bun run {0}",
    install: "bun install {0}",
    frozen: "bun install --no-save",
    global: "bun add -g {0}",
    add: "bun add {0}",
    upgrade: "bun update {0}",
    upgrade_interactive: "bun update {0}",
    execute: "bunx {0}",
    uninstall: "bun remove {0}",
    global_uninstall: "bun remove -g {0}",
};
