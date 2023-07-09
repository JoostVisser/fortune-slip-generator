use std::env;

use owo_colors::OwoColorize;

use crate::{cli::windows, fortune::FortuneGenerator};
pub mod fortune;

mod cli;
mod constants;
mod pdf;
mod svg;

fn main() {
    let cli_args = cli::execute();

    if cli_args.verbose {
        enable_logging();
    }

    println!("Generating fortunes...");
    let fortune_generator = FortuneGenerator::open(&cli_args.config).unwrap();

    println!("Generating PDF...");
    fortune_generator.generate_to_pdf(&cli_args.output).unwrap();

    println!(
        "{} PDF generated at '{}'",
        "Success!".green().bold(),
        &cli_args.output.display()
    );

    windows::press_a_key_to_continue_windows_only();
}

fn enable_logging() {
    env::set_var("RUST_LOG", "DEBUG");
    pretty_env_logger::init();
}
