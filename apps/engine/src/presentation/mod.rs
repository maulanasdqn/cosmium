#[expect(
    clippy::print_stdout,
    reason = "the CLI layer renders its output to stdout"
)]
pub mod cli;
pub mod http;
