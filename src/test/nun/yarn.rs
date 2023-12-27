#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn single_uninstall() {
        let (agent, args) = parse_nun(Agent::Yarn, vec!["axios".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["remove", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_nun(
            Agent::Yarn,
            vec!["axios".into(), "@types/node".into()],
            None,
        );
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["remove", "axios", "@types/node"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_nun(
            Agent::Yarn,
            vec!["axios".into(), "@types/node".into(), "-D".into()],
            None,
        );
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["remove", "axios", "@types/node", "-D"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_nun(Agent::Yarn, vec!["eslint".into(), "-g".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["global", "remove", "eslint"]);
    }
}
