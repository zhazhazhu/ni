#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn single_uninstall() {
        let (agent, args) = parse_nun(Agent::Bun, vec!["axios".into()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["remove", "axios"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_nun(Agent::Bun, vec!["axios".into(), "@types/node".into()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["remove", "axios", "@types/node"]);
    }
    #[test]
    fn global() {
        let (agent, args) = parse_nun(Agent::Bun, vec!["eslint".into(), "-g".into()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["remove", "-g", "eslint"]);
    }
}
