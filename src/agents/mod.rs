pub enum Agents {
    Npm,
    Yarn,
    YarnBerry,
    Pnpm,
    Pnpm6,
    Bun,
}

impl Agents {
    pub fn as_str(&self) -> &str {
        match self {
            Agents::Npm => "npm",
            Agents::Yarn => "yarn",
            Agents::YarnBerry => "yarn@berry",
            Agents::Pnpm => "pnpm",
            Agents::Pnpm6 => "pnpm@6",
            Agents::Bun => "bun",
        }
    }
}
