use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};

use super::svg_to_pdf;

#[derive(Debug, PartialEq, Eq)]
pub struct SvgFile {
    pub path: PathBuf,
}

impl SvgFile {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !check_valid_and_existing_svg(&path) {
            bail!("The path to the SVG file is not valid or does not exist");
        }

        Ok(Self { path })
    }

    pub fn to_pdf(&self, pdf_path: impl AsRef<Path>) -> Result<PathBuf> {
        svg_to_pdf::svg_to_pdf(&self.path, pdf_path.as_ref())?;

        return Ok(pdf_path.as_ref().to_path_buf());
    }

    pub fn to_pdf_same_name(&self) -> Result<PathBuf> {
        let pdf_path = self.path.with_extension("pdf");
        self.to_pdf(pdf_path)
    }
}

impl Display for SvgFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path.display())
    }
}

fn check_valid_and_existing_svg(path: &Path) -> bool {
    if path.exists() && path.is_file() {
        if let Some(extension) = path.extension() {
            if extension == "svg" {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use test_utils::create_temp_file;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_svg_file_new_with_valid_svg() {
        let temp_file = create_temp_file("test.svg", "test");
        let svg_file = SvgFile::new(&temp_file.path).unwrap();
        assert_eq!(svg_file.path, temp_file.path);
    }

    #[test]
    fn test_svg_file_new_with_txt_expect_error() {
        let temp_file = create_temp_file("test.txt", "test");
        let result = SvgFile::new(&temp_file.path);
        assert!(result.is_err());
    }

    #[test]
    fn test_svg_file_new_with_non_existent_svg_expect_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("nonexistent.svg");
        let result = SvgFile::new(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_svg_file_to_pdf() {
        let temp_file = create_temp_file("test.svg", "test");
        let pdf_path = temp_file.dir.path().join("hello.pdf");
        let svg_file = SvgFile::new(&temp_file.path).unwrap();
        let result = svg_file.to_pdf(&pdf_path);
        assert_eq!(result.unwrap(), pdf_path);
    }

    #[test]
    fn test_svg_file_to_pdf_same_name() {
        let temp_file = create_temp_file("test.svg", "test");
        let svg_file = SvgFile::new(&temp_file.path).unwrap();
        let result = svg_file.to_pdf_same_name();

        let mut pdf_path = temp_file.path.clone();
        pdf_path.set_extension("pdf");
        assert_eq!(result.unwrap(), pdf_path);
    }

    #[test]
    fn test_svg_file_display() {
        let temp_file = create_temp_file("test.svg", "test");
        let svg_file = SvgFile::new(temp_file.path.clone()).unwrap();
        assert_eq!(format!("{}", svg_file), temp_file.path.to_string_lossy());
    }
}
