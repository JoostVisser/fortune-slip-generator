use std::{path::Path, process::Command};

use anyhow::{Ok, Result};

pub fn svg_to_pdf(path_to_svg: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    let status = Command::new("inkscape")
        .arg("--export-area-drawing")
        .arg("--export-text-to-path")
        .arg("--batch-process")
        .arg("--export-type=pdf")
        .arg(format!(
            "--export-filename={}",
            output_path.as_ref().to_str().unwrap()
        ))
        .arg(path_to_svg.as_ref().to_str().unwrap())
        .status()
        .expect(
            format!(
                "Failed to convert {} to {}",
                path_to_svg.as_ref().to_str().unwrap(),
                output_path.as_ref().to_str().unwrap()
            )
            .as_str(),
        );

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "Failed to convert {} to {}",
            path_to_svg.as_ref().to_str().unwrap(),
            output_path.as_ref().to_str().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::svg_to_pdf;
    const SVG_EXAMPLE: &str = "<svg height='100' width='100'>
                                 <circle cx='50' cy='50' r='40' />
                               </svg>";

    #[test]
    fn convert_svg_expect_pdf() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let temp_pdf_path = temp_file.dir.path().join("temp.pdf");

        assert_eq!(temp_pdf_path.exists(), false);
        svg_to_pdf(&temp_file.path, &temp_pdf_path).unwrap();
        assert_eq!(temp_pdf_path.exists(), true);
    }
}
