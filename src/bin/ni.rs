use npack::parse::parse_ni;
use npack::runner::run_cli;

fn main() {
    run_cli(parse_ni, None)
}
