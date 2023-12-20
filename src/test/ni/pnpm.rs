#[cfg(test)]
mod pnpm {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_ni(Agent::Pnpm, Vec::new());
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["i"]);
    }
    #[test]
    fn single_add() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["axios".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["add", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["axios".to_string(), "eslint".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["add", "axios", "eslint"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_ni(
            Agent::Pnpm,
            vec!["axios".to_string(), "eslint".to_string(), "-D".to_string()],
        );
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["add", "axios", "eslint", "-D"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["axios".to_string(), "-g".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["add", "-g", "axios"]);
    }
    #[test]
    fn frozen() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["--frozen".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["i", "--frozen-lockfile"]);
    }
    #[test]
    fn forward1() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["--anything".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["i", "--anything"]);
    }
    #[test]
    fn forward2() {
        let (agent, args) = parse_ni(Agent::Pnpm, vec!["-a".to_string()]);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["i", "-a"]);
    }
}
