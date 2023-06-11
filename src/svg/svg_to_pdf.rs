use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Ok, Result};

pub fn svg_to_pdf(path_to_svg: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    let output_path_str = output_path
        .as_ref()
        .to_str()
        .ok_or(anyhow!("Output path is not valid unicode"))?;
    let path_to_svg_str = path_to_svg
        .as_ref()
        .to_str()
        .ok_or(anyhow!("Input path is not valid unicode"))?;

    println!("Converting {} to {}", path_to_svg_str, output_path_str);

    let parent_path = path_to_svg.as_ref().parent().unwrap();
    let parent_files = parent_path.read_dir().unwrap();
    for file in parent_files {
        let file = file.unwrap();
        let file_name = file.file_name();
        println!("File name: {:?}", file_name);
    }
    let status = execute_inkscape_command(path_to_svg_str, output_path_str)?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to convert {} to {}",
            path_to_svg.as_ref().to_str().unwrap(),
            output_path.as_ref().to_str().unwrap()
        ))
    }
}

fn execute_inkscape_command(
    path_to_svg: &str,
    output_path: &str,
) -> Result<std::process::ExitStatus> {
    println!("Input file path: {}", path_to_svg);
    println!("Export file name: {}", output_path);

    let mut command = Command::new("inkscape");
    let temp = command
        .arg("--export-area-drawing")
        .arg("--export-text-to-path")
        .arg("--export-type=pdf")
        .arg(format!("--export-filename={output_path}"))
        .arg(path_to_svg);

    println!("Args: {:?}", temp.get_args().collect::<Vec<_>>());

    let result = temp
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    println!(
        "Inkscape output
         StdOut: {}
         StdErr: {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );

    Ok(result.status)
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::svg_to_pdf;
    const SVG_EXAMPLE: &str = "<svg height='100' width='100'>
                                 <circle cx='50' cy='50' r='40' />
                               </svg>";

    #[test]
    fn convert_svg_expect_pdf() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let temp_pdf_path = temp_file.dir.path().join("temp.pdf");

        assert!(!temp_pdf_path.exists());
        svg_to_pdf(&temp_file.path, &temp_pdf_path).unwrap();
        assert!(temp_pdf_path.exists());
    }

    #[test]
    fn test_execute_inkscape_command() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let temp_pdf_path = temp_file.dir.path().join("temp.pdf");

        let status = super::execute_inkscape_command(
            temp_file.path.to_str().unwrap(),
            temp_pdf_path.to_str().unwrap(),
        )
        .unwrap();

        assert!(status.success());
    }

    #[test]
    fn simple_inkscape_test() {
        let result = Command::new("inkscape")
            .arg("--help")
            .status()
            .expect("Failed to execute process");
        assert!(result.success());
    }
}
