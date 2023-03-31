use crate::pdf::merge_pdf::merge_pdf;
use rand::{seq::SliceRandom, thread_rng};

pub mod pdf;
pub mod svg;
pub mod fortune;

fn main() {
    println!("Hello, world!");

    let mut nums = [1, 2, 3, 4, 5];
    let mut rng = thread_rng();

    nums.shuffle(&mut rng);

    println!("Numbers: {:?}", nums);

    println!("Generating frontside fortune...");
    // svg_to_pdf(
    //     "data/fortune_template/omikuji_frontside_template.svg",
    //     "data/fortune_output/omikuji_frontside.pdf",
    // );

    // println!("Generating backside fortune...");
    // svg_to_pdf(
    //     "data/fortune_template/omikuji_backside_1.svg",
    //     "data/fortune_output/omikuji_backside.pdf",
    // );

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
