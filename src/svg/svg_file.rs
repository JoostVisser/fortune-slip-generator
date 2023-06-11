use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use log::debug;

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

        debug!(
            "Converting '{}' to '{}'",
            self.path.display(),
            pdf_path.as_ref().display()
        );

        return Ok(pdf_path.as_ref().to_path_buf());
    }

    pub fn to_pdf_same_name(&self) -> Result<PathBuf> {
        let pdf_path = self.path.with_extension("pdf");
        self.to_pdf(pdf_path)
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

impl Display for SvgFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path.display())
    }
}
