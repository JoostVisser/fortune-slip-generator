use crate::file_manip::{merge_pdf::merge_pdf, svg_converter::convert_svg_to_pdf};
use rand::{seq::SliceRandom, thread_rng};

pub mod file_manip;
pub mod fortunes;

fn main() {
    println!("Hello, world!");

    let mut nums = [1, 2, 3, 4, 5];
    let mut rng = thread_rng();

    nums.shuffle(&mut rng);

    println!("Numbers: {:?}", nums);

    println!("Generating frontside fortune...");
    convert_svg_to_pdf(
        "data/fortune_template/omikuji_frontside_template.svg",
        "data/fortune_output/omikuji_frontside.pdf",
    );

    println!("Generating backside fortune...");
    convert_svg_to_pdf(
        "data/fortune_template/omikuji_backside_1.svg",
        "data/fortune_output/omikuji_backside.pdf",
    );

    merge_pdf(
        [
            "data/fortune_output/omikuji_frontside.pdf",
            "data/fortune_output/omikuji_backside.pdf",
            "data/fortune_output/omikuji_frontside.pdf",
            "data/fortune_output/omikuji_backside.pdf",
        ],
        "data/fortune_output/omikuji_merged.pdf",
    )
    .expect("Could not convert to pdf");
}
