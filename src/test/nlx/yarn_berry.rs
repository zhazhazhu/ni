#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn single_uninstall() {
        let (agent, args) = parse_nlx(Agent::YarnBerry, vec!["esbuild".into()], None);
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["dlx", "esbuild"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_nlx(
            Agent::YarnBerry,
            vec!["esbuild".into(), "--version".into()],
            None,
        );
        assert_eq!(agent, "yarn");
        assert_eq!(args, ["dlx", "esbuild", "--version"]);
    }
}
