#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_ni(Agent::Bun, Vec::new(), None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["install"]);
    }
    #[test]
    fn single_add() {
        let (agent, args) = parse_ni(Agent::Bun, vec!["axios".to_string()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["add", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_ni(
            Agent::Bun,
            vec!["axios".to_string(), "eslint".to_string()],
            None,
        );
        assert_eq!(agent, "bun");
        assert_eq!(args, ["add", "axios", "eslint"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_ni(
            Agent::Bun,
            vec!["axios".to_string(), "eslint".to_string(), "-D".to_string()],
            None,
        );
        assert_eq!(agent, "bun");
        assert_eq!(args, ["add", "axios", "eslint", "-d"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_ni(
            Agent::Bun,
            vec!["axios".to_string(), "-g".to_string()],
            None,
        );
        assert_eq!(agent, "bun");
        assert_eq!(args, ["add", "-g", "axios"]);
    }
    #[test]
    fn frozen() {
        let (agent, args) = parse_ni(Agent::Bun, vec!["--frozen".to_string()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["install", "--no-save"]);
    }
}
