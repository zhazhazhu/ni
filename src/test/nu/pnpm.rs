#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nu(Agent::Pnpm, vec![], None);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["update"]);
    }
    #[test]
    fn interactive() {
        let (agent, args) = parse_nu(Agent::Pnpm, vec!["-i".into()], None);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["update", "-i"]);
    }
    #[test]
    fn interactive_latest() {
        let (agent, args) = parse_nu(Agent::Pnpm, vec!["-i".into(), "--latest".into()], None);
        assert_eq!(agent, "pnpm");
        assert_eq!(args, ["update", "-i", "--latest"]);
    }
}
