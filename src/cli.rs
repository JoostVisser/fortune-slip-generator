use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};
use figlet_rs::FIGfont;
use owo_colors::{OwoColorize, Stream};

use crate::{
    cli::checks::check_prerequisites,
    constants::{DEFAULT_OUTPUT_PATH, DEFAULT_SETTINGS_PATH},
};

mod checks;
pub mod windows;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Path to the output PDF.
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_OUTPUT_PATH)]
    pub output: PathBuf,

    /// Custom path to the settings YAML file.
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_SETTINGS_PATH)]
    pub config: PathBuf,

    /// Skip the prerequisites checks.
    #[arg(short, long)]
    pub skip_checks: bool,

    /// Print logs to the console.
    #[arg(short, long)]
    pub verbose: bool,
}

/// Parses the CLI arguments and returns the write options.
pub fn execute() -> Result<CliArgs> {
    windows::enable_ansi_support();

    let cli = CliArgs::parse();
    print_logo();
    println!("Welcome to the fortune slips generator!");
    println!();
    if !cli.skip_checks {
        check_prerequisites(&cli.config)?;
    }

    Ok(cli)
}

fn print_logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Fortune  slips").unwrap();

    println!(
        "{}",
        figure.if_supports_color(Stream::Stdout, |text| text.bold())
    );
}
