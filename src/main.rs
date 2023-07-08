use std::process::Command;

use owo_colors::OwoColorize;

use crate::fortune::FortuneGenerator;
pub mod fortune;

mod cli;
mod constants;
mod pdf;
mod svg;
mod write_options;

fn main() {
    pretty_env_logger::init();

    let write_options = cli::execute();

    println!("Generating fortunes...");
    let fortune_generator = FortuneGenerator::open(&write_options.config_path).unwrap();

    println!("Generating PDF...");
    fortune_generator
        .generate_to_pdf(&write_options.output_path)
        .unwrap();

    println!(
        "{} PDF generated at '{}'",
        "Success!".green().bold(),
        write_options.output_path.display()
    );

    if cfg!(target_os = "windows") {
        println!("");
        println!("Press any key to exit...");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
}
