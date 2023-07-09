//! # Fortune Slips Generator
//! This crate generates a PDF with fortune slips.
//!
//! Fortune slips are small pieces of paper with a fortune written on them.
//!
//! ## Usage
//! ```rust
//! # use anyhow::Ok;
//! use fortune_generator::FortuneGenerator;
//!
//! let fortune_gen = FortuneGenerator::open("fortune_settings.yaml")?;
//! fortune_gen.generate_to_pdf("fortune_slips.pdf")?; // Generates a PDF with fortune slips
//! # std::fs::remove_file("fortune_slips.pdf")?;
//! # Ok(())
//! ```
use std::env;

use owo_colors::OwoColorize;

use crate::{cli::windows, error::Error};

mod cli;
mod constants;
mod pdf;
mod svg;

pub mod error;
pub mod fortune;

pub use crate::fortune::fortune_data;
pub use crate::fortune::fortune_splitter;
pub use crate::fortune::FortuneGenerator;

pub fn run() -> Result<(), Error> {
    let cli_args = cli::execute().map_err(|_| Error::ChecksFailed)?;

    if cli_args.verbose {
        enable_logging();
    }

    println!("Generating fortunes...");
    let fortune_generator = FortuneGenerator::open(&cli_args.config)
        .map_err(|e| Error::FortuneSettingsLoadFailure(e.to_string()))?;

    println!("Generating PDF...");
    fortune_generator
        .generate_to_pdf(&cli_args.output)
        .map_err(|e| Error::PdfGenerateFailure(e.to_string()))?;

    println!();
    println!(
        "{} PDF generated at '{}'",
        "Success!".green().bold(),
        &cli_args.output.display()
    );

    windows::press_a_key_to_continue_windows_only();
    Ok(())
}

fn enable_logging() {
    env::set_var("RUST_LOG", "DEBUG");
    pretty_env_logger::init();
}
