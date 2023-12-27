#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn single_uninstall() {
        let (agent, args) = parse_nun(Agent::Npm, vec!["axios".into()], None);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["uninstall", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_nun(Agent::Npm, vec!["axios".into(), "@types/node".into()], None);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["uninstall", "axios", "@types/node"]);
    }
    #[test]
    fn dependencies() {
        let (agent, args) = parse_nun(
            Agent::Npm,
            vec!["axios".into(), "@types/node".into(), "-D".into()],
            None,
        );
        assert_eq!(agent, "npm");
        assert_eq!(args, ["uninstall", "axios", "@types/node", "-D"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_nun(Agent::Npm, vec!["eslint".into(), "-g".into()], None);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["uninstall", "-g", "eslint"]);
    }
}
