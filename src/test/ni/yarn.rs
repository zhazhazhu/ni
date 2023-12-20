#[cfg(test)]
mod yarn {
    use crate::agents::*;
    use crate::parse::*;
    #[test]
    fn empty() {
        let (agent, args) = parse_ni(Agent::Yarn, Vec::new());
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["install"]);
    }
    #[test]
    fn single_add() {
        let (agent, args) = parse_ni(Agent::Yarn, vec!["axios".to_string()]);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["add", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_ni(Agent::Yarn, vec!["axios".to_string(), "eslint".to_string()]);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["add", "axios", "eslint"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_ni(
            Agent::Yarn,
            vec!["axios".to_string(), "eslint".to_string(), "-D".to_string()],
        );
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["add", "axios", "eslint", "-D"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_ni(Agent::Yarn, vec!["axios".to_string(), "-g".to_string()]);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["global", "add", "axios"]);
    }
    #[test]
    fn frozen() {
        let (agent, args) = parse_ni(Agent::Yarn, vec!["--frozen".to_string()]);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["install", "--frozen-lockfile"]);
    }
}
