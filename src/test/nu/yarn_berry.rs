#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nu(Agent::YarnBerry, vec![], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["up"]);
    }
    #[test]
    fn interactive() {
        let (agent, args) = parse_nu(Agent::YarnBerry, vec!["-i".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["up", "-i"]);
    }
}
