use std::path::Path;

use anyhow::Result;

use super::FortuneSlip;

pub struct WriteOptions {
    pub print_on_long_side: bool,
}

impl WriteOptions {
    /// Create a new WriteOptions with default configurations.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            print_on_long_side: true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FortuneWriter<'a> {
    fortunes: Vec<&'a FortuneSlip>,
}

impl<'a> FortuneWriter<'a> {
    pub fn new() -> Self {
        FortuneWriter { fortunes: vec![] }
    }

    pub fn add_fortune(&mut self, fortune_slip: &'a FortuneSlip) {
        self.fortunes.push(fortune_slip);
    }

    pub fn add_fortunes(&mut self, fortune_slips: impl IntoIterator<Item = &'a FortuneSlip>) {
        self.fortunes.extend(fortune_slips);
    }

    /// Writes the fortunes to a folder as PDF files.
    /// Since only 4 slips fit on a page, every 4 slips will generate a new PDF file.
    ///
    pub fn write(
        &self,
        pdf_path: impl AsRef<Path>,
        write_options: Option<WriteOptions>,
    ) -> Result<()> {
        // Step 1: Convert fortunes into SVG element objects
        // Step 2: Write SVG element objects per 4 into a temporary file.
        // Step 3: Merge the PDFs into a single PDF

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;
    use tempfile::TempDir;

    use crate::fortune::FortuneSlip;

    use super::FortuneWriter;

    #[test]
    fn test_add_fortune() {
        let fortune = get_standard_furtune();
        let mut fortune_writer = FortuneWriter::new();
        fortune_writer.add_fortune(&fortune);

        assert_eq!(fortune_writer.fortunes, vec![&fortune])
    }

    #[test]
    fn test_add_fortunes() {
        let fortune_1 = get_standard_furtune();
        let fortune_2 = fortune_1.clone();

        let mut fortune_writer = FortuneWriter::new();
        fortune_writer.add_fortunes(vec![&fortune_1, &fortune_2]);

        assert_eq!(fortune_writer.fortunes, vec![&fortune_1, &fortune_2])
    }

    #[test]
    fn test_write_single_slip_expect_1_pdf() -> Result<()> {
        let pdf_path = TempDir::new()?.path().join("temp.pdf");

        let fortune = get_standard_furtune();
        let mut fortune_writer = FortuneWriter::new();
        fortune_writer.add_fortune(&fortune);
        fortune_writer.write(&pdf_path, None)?;

        assert!(pdf_path.exists());

        Ok(())
    }

    #[test]
    fn test_write_multiple_slips_expect_1_pdf() -> Result<()> {
        let pdf_path = TempDir::new()?.path().join("temp.pdf");

        let fortune_slips = vec![get_standard_furtune(); 8];
        let mut fortune_writer = FortuneWriter::new();
        for fortune in &fortune_slips {
            fortune_writer.add_fortune(fortune);
        }

        fortune_writer.write(&pdf_path, None)?;

        assert!(pdf_path.exists());

        Ok(())
    }

    fn get_standard_furtune() -> FortuneSlip {
        let mut fortune_categories = HashMap::new();
        fortune_categories.insert(
            "general".to_string(),
            "You will live a prosperous live!".to_string(),
        );
        fortune_categories.insert("love".to_string(), "You will find love!".to_string());

        FortuneSlip {
            fortune_header: "Great luck".to_string(),
            fortune_luck_level: "大福".to_string(),
            fortune_categories,
        }
    }
}
