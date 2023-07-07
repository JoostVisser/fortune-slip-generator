use std::path::PathBuf;

use clap::{command, Parser};
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;

use crate::{
    cli::checks::check_prerequisites,
    constants::{DEFAULT_OUTPUT_PATH, DEFAULT_SETTINGS_PATH},
    write_options::WriteOptions,
};

mod checks;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    /// Path to the output PDF.
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_OUTPUT_PATH)]
    output: PathBuf,

    /// Custom path to the settings YAML file.
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_SETTINGS_PATH)]
    config: PathBuf,

    /// Skip the prerequisites checks.
    #[arg(short, long)]
    skip_checks: bool,
}

pub fn execute() -> WriteOptions {
    let cli = CliArgs::parse();
    print_logo();
    println!("Welcome to the fortune slips generator!");
    println!();
    if !cli.skip_checks {
        check_prerequisites(&cli.config);
    }

    WriteOptions {
        output_path: cli.output,
        config_path: cli.config,
    }
}

pub fn print_logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Fortune  slips").unwrap();
    println!("{}", figure.bold());
}
