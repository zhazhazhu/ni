use npack::{
    parse::parse_ni,
    runner::{run_cli, DetectOptions},
};

fn main() {
    run_cli(
        |agent, _, ctx| parse_ni(agent, vec!["--frozen-if-present".into()], ctx),
        Some(DetectOptions::new().with_auto_install(true)),
    )
}
