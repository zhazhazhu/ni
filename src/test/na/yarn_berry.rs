#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_na(Agent::YarnBerry, vec![], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args.len(), 0);
    }
    #[test]
    fn foo() {
        let (agent, args) = parse_na(Agent::YarnBerry, vec!["foo".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["foo"]);
    }
    #[test]
    fn run_test() {
        let (agent, args) = parse_na(Agent::YarnBerry, vec!["run".into(), "test".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["run", "test"]);
    }
}
