use log::info;

use crate::fortune::FortuneGenerator;

pub mod fortune;

mod constants;
mod pdf;
mod svg;

fn main() {
    pretty_env_logger::init();

    let fortune_settings = "data/fortune_data/settings.yaml";
    info!("Loading data from '{}'", fortune_settings);
    let a = FortuneGenerator::open(fortune_settings).unwrap();

    let output_pdf_path = "test.pdf";
    info!("Generating fortune slips to '{}'...", output_pdf_path);
    a.generate_to_pdf("test.pdf").unwrap();
    info!("Done!");
}
