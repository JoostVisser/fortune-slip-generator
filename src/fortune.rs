use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use itertools::intersperse;
use log::debug;
use rayon::prelude::*;
use tempfile::TempDir;

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

mod fortune_data;
mod fortune_slip_writer;
mod fortune_splitter;

pub struct FortuneGenerator {
    fortune_data: FortuneData,
}

impl FortuneGenerator {
    pub fn open(settings_yaml_path: impl AsRef<Path>) -> Result<FortuneGenerator> {
        let fortune_data = FortuneData::open(settings_yaml_path)?;

        Ok(FortuneGenerator { fortune_data })
    }

    /// Writes the fortunes to target PDF file.
    ///
    /// Example:
    /// ```
    /// let fortune_generator = FortuneGenerator::open("test_utils/data/fortune_settings.yaml")?;
    /// fortune_generator.generate_to_pdf("test.pdf")?;
    /// ```
    pub fn generate_to_pdf(&self, pdf_path: impl AsRef<Path>) -> Result<()> {
        if pdf_path.as_ref().is_dir() {
            bail!("The path to write the PDF file cannot be a directory");
        }

        let fortunes = self.get_random_fortunes()?;

        let temp_dir = TempDir::new()?;

        let front_pdf_paths = self.generate_pdf_fortunes(temp_dir.path(), fortunes)?;
        let backside_pdf_path = self.generate_backside_pdf(temp_dir.path())?;

        Self::intersperse_and_merge_pdfs(front_pdf_paths, backside_pdf_path, pdf_path)
    }

    /// Writes the fortunes to a folder as SVG files.
    /// Since only 4 slips fit on a page, every 4 slips will generate a new SVG file.
    ///
    /// Returns:
    ///     A vector of the paths to the SVG files.
    pub fn generate_to_svg_dir(&self, svg_dir: impl AsRef<Path>) -> Result<Vec<SvgFile>> {
        let svg_dir = svg_dir.as_ref();

        if !svg_dir.is_dir() {
            bail!("The path to write the SVG files to is not a directory, but a file")
        }

        let fortunes = self.get_random_fortunes()?;
        let svg_paths = self.save_fortunes_to_svg(&fortunes, svg_dir)?;

        Ok(svg_paths)
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
        merge_pdf::merge_pdf(&all_pdf_paths, pdf_path)
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
        svg_paths
            .par_iter()
            .map(|svg_path| svg_path.to_pdf_same_name())
            .collect::<Result<Vec<_>>>()
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
    use std::{fs, path::Path};

    use anyhow::Result;
    use lopdf::Document;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    use crate::{fortune::FortuneGenerator, svg::svg_file::SvgFile};

    #[fixture]
    #[once]
    fn fortune_generator() -> FortuneGenerator {
        FortuneGenerator::open("test_utils/data/fortune_settings.yaml").unwrap()
    }

    #[rstest]
    fn test_open(_fortune_generator: &FortuneGenerator) {}

    #[rstest]
    fn test_generate_to_pdf_with_directory_path(fortune_generator: &FortuneGenerator) {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = fortune_generator.generate_to_pdf(temp_dir.path());
        assert!(result.is_err());
    }

    #[rstest]
    fn test_generate_to_pdf(fortune_generator: &FortuneGenerator) -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
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
    fn test_generate_to_svg_dir(fortune_generator: &FortuneGenerator) -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let svg_paths = fortune_generator.generate_to_svg_dir(&temp_dir.path())?;

        let paths = fs::read_dir(temp_dir.path())?;

        let mut nr_svg_files = 0;
        for path in paths {
            let path_buf = path?.path();
            assert_eq!(path_buf.extension().unwrap(), "svg");
            let svg_file = SvgFile::new(&path_buf)?;
            assert!(svg_paths.contains(&svg_file));
            assert_eq!(path_buf.parent().unwrap(), temp_dir.path());

            nr_svg_files += 1;
        }
        assert_eq!(nr_svg_files, 2);

        Ok(())
    }

    #[rstest]
    fn test_generate_to_pdf_folder_error(fortune_generator: &FortuneGenerator) {
        let temp_dir = tempfile::tempdir().unwrap();
        let generation_result = fortune_generator.generate_to_pdf(&temp_dir.path());

        assert!(generation_result.is_err());
    }

    #[rstest]
    fn test_generate_to_svg_file_error(fortune_generator: &FortuneGenerator) {
        let temp_dir = tempfile::tempdir().unwrap();
        let svg_path = temp_dir.path().join("fortunes.svg");
        let generation_result = fortune_generator.generate_to_svg_dir(&svg_path);

        assert!(generation_result.is_err());
    }
}
