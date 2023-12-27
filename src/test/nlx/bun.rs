#[cfg(test)]
mod bun {
    use crate::agents::*;
    use crate::parse::*;

    #[test]
    fn single_uninstall() {
        let (agent, args) = parse_nlx(Agent::Bun, vec!["esbuild".into()], None);
        assert_eq!(agent, "bunx");
        assert_eq!(args, ["esbuild"]);
    }
    #[test]
    fn multiple() {
        let (agent, args) = parse_nlx(Agent::Bun, vec!["esbuild".into(), "--version".into()], None);
        assert_eq!(agent, "bunx");
        assert_eq!(args, ["esbuild", "--version"]);
    }
}
