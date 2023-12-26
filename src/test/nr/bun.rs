#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nr(Agent::Bun, Vec::new());
        assert_eq!(agent, "bun");
        assert_eq!(args, ["run", "start"]);
    }
    #[test]
    fn script() {
        let (agent, args) = parse_nr(Agent::Bun, vec!["dev".into()]);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["run", "dev"]);
    }
    #[test]
    fn script_with_arguments() {
        let (agent, args) = parse_nr(
            Agent::Bun,
            vec!["build".into(), "--watch".into(), "-o".into()],
        );
        assert_eq!(agent, "bun");
        assert_eq!(args, ["run", "build", "--watch", "-o"]);
    }
    #[test]
    fn colon() {
        let (agent, args) = parse_nr(Agent::Bun, vec!["build:dev".into()]);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["run", "build:dev"]);
    }
}
