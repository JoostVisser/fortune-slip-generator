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
    use std::fs;

    use super::*;

    #[test]
    fn test_check_valid_and_existing_svg_with_valid_svg() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.svg");
        fs::write(&path, "test").unwrap();
        assert_eq!(check_valid_and_existing_svg(&path), true);
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_check_valid_and_existing_svg_with_invalid_svg() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");
        fs::write(&path, "test").unwrap();
        assert_eq!(check_valid_and_existing_svg(&path), false);
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_check_valid_and_existing_svg_with_nonexistent_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("nonexistent.svg");
        assert_eq!(check_valid_and_existing_svg(&path), false);
    }

    #[test]
    fn test_svg_file_display() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.svg");
        fs::write(&path, "test").unwrap();
        let svg_file = SvgFile::new(path.clone()).unwrap();
        assert_eq!(format!("{}", svg_file), path.to_string_lossy());
    }
}
