#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nu(Agent::Bun, vec![], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["update"]);
    }
    #[test]
    fn interactive() {
        let (agent, args) = parse_nu(Agent::Bun, vec!["-i".into()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["update"]);
    }
    #[test]
    fn interactive_latest() {
        let (agent, args) = parse_nu(Agent::Bun, vec!["-i".into(), "--latest".into()], None);
        assert_eq!(agent, "bun");
        assert_eq!(args, ["update", "--latest"]);
    }
}
