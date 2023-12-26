use npack::{parse::parse_nlx, runner::run_cli};

fn main() {
    run_cli(parse_nlx, None)
}
