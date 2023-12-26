use npack::{parse::parse_nu, runner::run_cli};

fn main() {
    run_cli(parse_nu, None)
}
