use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use itertools::intersperse;
use log::{debug, info};
use rayon::prelude::*;
use tempfile::tempdir;

use crate::{
    constants::NR_SLIPS_PER_PAGE,
    pdf::merge_pdf,
    svg::{svg_editor::SvgEditor, svg_file::SvgFile},
};

use self::{
    fortune_data::FortuneData,
    fortune_slip_writer::FortuneSlipWriter,
    fortune_splitter::{FortuneSlipTextRef, FortuneSplitter},
};

pub mod fortune_data;
mod fortune_slip_writer;
pub mod fortune_splitter;

pub struct FortuneGenerator {
    fortune_data: FortuneData,
}

impl FortuneGenerator {
    /// Opens the fortune settings file and returns a `FortuneGenerator` instance.
    pub fn open(settings_yaml_path: impl AsRef<Path>) -> Result<FortuneGenerator> {
        let fortune_data = FortuneData::open(settings_yaml_path)?;

        Ok(FortuneGenerator { fortune_data })
    }

    /// Writes the fortunes to target PDF file.
    ///
    /// Example:
    /// ```
    /// # use anyhow::Ok;
    /// use fortune_generator::fortune::FortuneGenerator;
    ///
    /// let fortune_gen = FortuneGenerator::open("test_utils/data/fortune_settings.yaml")?;
    /// fortune_gen.generate_to_pdf("output.pdf"); // Writes the fortunes to output.pdf
    /// # std::fs::remove_file("output.pdf").unwrap();
    /// # Ok(())
    /// ```
    ///
    pub fn generate_to_pdf(&self, pdf_path: impl AsRef<Path>) -> Result<()> {
        if pdf_path.as_ref().is_dir() {
            bail!("The path to write the PDF file cannot be a directory");
        }

        let fortunes = self.get_random_fortunes()?;
        println!("Writing {} fortunes...", fortunes.len());

        let temp_dir = tempdir()?;

        let front_pdf_paths = self.generate_pdf_fortunes(temp_dir.path(), fortunes)?;
        let backside_pdf_path = self.generate_backside_pdf(temp_dir.path())?;

        Self::intersperse_and_merge_pdfs(front_pdf_paths, backside_pdf_path, pdf_path)
    }

    fn get_random_fortunes(&self) -> Result<Vec<FortuneSlipTextRef>> {
        let fortune_splitter = FortuneSplitter::new(&self.fortune_data);
        fortune_splitter.shuffle_and_split()
    }

    fn generate_pdf_fortunes(
        &self,
        dir: &Path,
        fortunes: Vec<FortuneSlipTextRef>,
    ) -> Result<Vec<PathBuf>> {
        let svg_files = self.save_fortunes_to_svg(&fortunes, dir)?;
        let front_pdf_paths = self.convert_svg_to_pdf_same_dir(&svg_files)?;
        Ok(front_pdf_paths)
    }

    fn generate_backside_pdf(&self, dir: &Path) -> Result<PathBuf> {
        let backside_template_path = &self.fortune_data.get_settings().template_back;
        let backside_svg_file = SvgFile::new(backside_template_path)?;

        let target_path = dir.join("backside.pdf");
        backside_svg_file.to_pdf(target_path)
    }

    fn intersperse_and_merge_pdfs(
        front_pdf_paths: Vec<PathBuf>,
        backside_pdf_path: PathBuf,
        pdf_path: impl AsRef<Path>,
    ) -> Result<(), anyhow::Error> {
        let mut all_pdf_paths =
            intersperse(front_pdf_paths, backside_pdf_path.clone()).collect::<Vec<_>>();
        all_pdf_paths.push(backside_pdf_path);
        merge_pdf(&all_pdf_paths, pdf_path)
    }

    fn save_fortunes_to_svg(
        &self,
        fortune_slip_texts: &[FortuneSlipTextRef],
        svg_dir: impl AsRef<Path>,
    ) -> Result<Vec<SvgFile>> {
        let svg_dir = svg_dir.as_ref();
        let fortune_text_all_pages = fortune_slip_texts.chunks(NR_SLIPS_PER_PAGE);

        let mut single_slip_writer = self.open_single_slip_writer()?;
        let mut svg_files = vec![];

        for (i, fortune_text_page) in fortune_text_all_pages.enumerate() {
            let svg_path = svg_dir.join(format!("{}.svg", i));
            debug!("Writing page #{} to '{}'", i, svg_path.display());
            single_slip_writer.write_page(fortune_text_page)?;
            single_slip_writer.save_to_svg(&svg_path)?;
            svg_files.push(SvgFile::new(svg_path)?);
        }

        Ok(svg_files)
    }

    fn convert_svg_to_pdf_same_dir(&self, svg_paths: &[SvgFile]) -> Result<Vec<PathBuf>> {
        info!("Converting SVG files to PDF... (can take a while)");
        let result = svg_paths
            .par_iter()
            .map(|svg_path| svg_path.to_pdf_same_name())
            .collect::<Result<Vec<_>>>();
        result
    }

    fn open_single_slip_writer(&self) -> Result<FortuneSlipWriter> {
        let template_front = &self.fortune_data.get_settings().template_front;
        let svg_editor = SvgEditor::open(template_front)?;
        let fortune_categories = self
            .fortune_data
            .get_categories()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        FortuneSlipWriter::new(svg_editor, &fortune_categories)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use lopdf::Document;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use tempfile::tempdir;

    use crate::fortune::FortuneGenerator;

    #[fixture]
    fn fortune_generator() -> FortuneGenerator {
        FortuneGenerator::open("test_utils/data/fortune_settings.yaml").unwrap()
    }

    #[rstest]
    fn test_open(_fortune_generator: FortuneGenerator) {}

    #[rstest]
    fn test_generate_to_pdf_with_directory_path(fortune_generator: FortuneGenerator) {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = fortune_generator.generate_to_pdf(temp_dir.path());
        assert!(result.is_err());
    }

    #[rstest]
    fn test_generate_to_pdf(fortune_generator: FortuneGenerator) -> Result<()> {
        let temp_dir = tempdir()?;
        let pdf_path = temp_dir.path().join("fortunes.pdf");
        fortune_generator.generate_to_pdf(&pdf_path)?;

        assert!(pdf_path.exists());

        let nr_pdf_pages = open_pdf_and_count_pages(pdf_path)?;

        // Because there are a total of 5 slips and 4 slips a page, there will be 2 front pages.
        // Each page also has a backside, so there should be 4 pages in total.
        assert_eq!(nr_pdf_pages, 4);

        Ok(())
    }

    fn open_pdf_and_count_pages(pdf_path: impl AsRef<Path>) -> Result<usize> {
        let doc = Document::load(pdf_path)?;
        let pages = doc.get_pages();
        Ok(pages.len())
    }

    #[rstest]
    fn test_generate_to_pdf_folder_error(fortune_generator: FortuneGenerator) {
        let temp_dir = tempfile::tempdir().unwrap();
        let generation_result = fortune_generator.generate_to_pdf(temp_dir.path());

        assert!(generation_result.is_err());
    }
}
