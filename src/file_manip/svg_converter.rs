use std::{path::Path, process::Command};

pub fn convert_svg_to_pdf<P: AsRef<Path>>(path_to_svg: P, output_path: P) {
    Command::new("inkscape")
        .arg("--export-area-drawing")
        .arg("--export-text-to-path")
        .arg("--batch-process")
        .arg("--export-type=pdf")
        .arg(format!(
            "--export-filename={}",
            output_path.as_ref().to_str().unwrap()
        ))
        .arg(path_to_svg.as_ref().to_str().unwrap())
        .output()
        .expect("failed to execute process");
}
