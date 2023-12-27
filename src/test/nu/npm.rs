#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn empty() {
        let (agent, args) = parse_nu(Agent::Npm, vec![], None);
        assert_eq!(agent, "npm");
        assert_eq!(args, ["update"]);
    }
}
