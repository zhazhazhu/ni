#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nr(Agent::Npm, Vec::new());
        assert_eq!(agent, "npm");
        assert_eq!(args, ["run", "start"]);
    }
    #[test]
    fn if_present() {
        let (agent, args) = parse_nr(Agent::Npm, vec!["test".into(), "--if-present".into()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["run", "--if-present", "test"]);
    }
    #[test]
    fn script() {
        let (agent, args) = parse_nr(Agent::Npm, vec!["dev".into()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["run", "dev"]);
    }
    #[test]
    fn script_with_arguments() {
        let (agent, args) = parse_nr(
            Agent::Npm,
            vec!["build".into(), "--watch".into(), "-o".into()],
        );
        assert_eq!(agent, "npm");
        assert_eq!(args, ["run", "build", "--", "--watch", "-o"]);
    }
    #[test]
    fn colon() {
        let (agent, args) = parse_nr(Agent::Npm, vec!["build:dev".into()]);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["run", "build:dev"]);
    }
}
