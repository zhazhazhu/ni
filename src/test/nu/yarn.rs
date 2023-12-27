#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nu(Agent::Yarn, vec![], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["upgrade"]);
    }
    #[test]
    fn interactive() {
        let (agent, args) = parse_nu(Agent::Yarn, vec!["-i".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["upgrade-interactive"]);
    }
    #[test]
    fn interactive_latest() {
        let (agent, args) = parse_nu(Agent::Yarn, vec!["-i".into(), "--latest".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["upgrade-interactive", "--latest"]);
    }
}
