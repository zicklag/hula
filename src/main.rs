//! A lua implementation build on [HVM].
//!
//! [HVM]: https://github.com/HigherOrderCO/HVM/

use std::path::PathBuf;

use clap::{builder::styling, Parser};
use once_cell::sync::Lazy;

/// Parallel lua implementation built on the HVM.
#[derive(clap::Parser)]
#[command(author, version, about, styles = cli_styles())]
struct Args {
    /// The lua script to run.
    script: PathBuf,
}

/// CLI arguments static.
static ARGS: Lazy<Args> = Lazy::new(Args::parse);

/// CLI output styling.
fn cli_styles() -> clap::builder::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

fn main() {
    // Parse commandline arguments
    Lazy::force(&ARGS);
}
