#[cfg(test)]
mod npm {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_ni(Agent::Npm, Vec::new());
        assert_eq!(agent, "npm");
        assert_eq!(args, ["i"]);
    }
    #[test]
    fn single_add() {
        let (agent, args) = parse_ni(Agent::Npm, vec!["axios".to_string()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["i", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_ni(Agent::Npm, vec!["axios".to_string(), "eslint".to_string()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["i", "axios", "eslint"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_ni(
            Agent::Npm,
            vec!["axios".to_string(), "eslint".to_string(), "-D".to_string()],
        );
        assert_eq!(agent, "npm");
        assert_eq!(args, ["i", "axios", "eslint", "-D"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_ni(Agent::Npm, vec!["axios".to_string(), "-g".to_string()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["i", "-g", "axios"]);
    }
    #[test]
    fn frozen() {
        let (agent, args) = parse_ni(Agent::Npm, vec!["--frozen".to_string()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["ci"]);
    }
}
