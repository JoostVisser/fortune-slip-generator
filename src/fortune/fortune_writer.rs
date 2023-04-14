use self::fortune_slip_svg::FortuneSlipSvg;
use crate::{
    fortune::fortune_writer::elem_utils::ElemUtils, svg::svg_editor::{SvgEditor, text_elem::TextElem},
};
use anyhow::{bail, Result};
use xmltree::Element;
use std::{collections::HashMap, path::Path, vec};

use super::fortune_slips::FortuneSlip;

mod elem_utils;
mod fortune_slip_svg;

const NR_SLIPS_PER_PAGE: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
struct FortuneWriter {
    svg_editor: SvgEditor,
    svg_elems_original: Vec<FortuneSlipSvg>,
    svg_elems_modified: Vec<FortuneSlipSvg>,
}

impl FortuneWriter {
    pub fn load(mut svg_editor: SvgEditor) -> Result<Self> {
        let svg_elems_original = Self::get_svg_elems_from_svg_file(&mut svg_editor)?;

        Ok(FortuneWriter {
            svg_editor,
            svg_elems_original,
            svg_elems_modified: vec![],
        })
    }

    fn get_svg_elems_from_svg_file(svg_editor: &mut SvgEditor) -> Result<Vec<FortuneSlipSvg>> {
        let mut all_text_elems = svg_editor.get_text_elems_mut();
        let text_elems_per_slip = all_text_elems.len() / NR_SLIPS_PER_PAGE;

        all_text_elems
            .chunks_exact(text_elems_per_slip)
            .map(Self::create_fortune_svg_element)
            .collect()
    }

    fn create_fortune_svg_element(svg_elements: &[&mut TextElem]) -> Result<FortuneSlipSvg> {
        todo!()
        // let mut header = None;
        // let mut luck_level = None;
        // let mut category_to_fortune = HashMap::new();

        // for svg_element in svg_elements {
        //     // println!("Svg element: {:?}", svg_element.any_text());
        //     match svg_element.text {
        //         Some(text) if text.contains("header") => header = Some(svg_element),
        //         Some(text) if text.contains("luck_level") => luck_level = Some(svg_element),
        //         Some(text) if text.contains("_") && text.contains(".") => {
        //             let category = text.split_once("_").unwrap();
        //             category_to_fortune.insert(category.0.to_string(), svg_element);
        //         }
        //         Some(_) => (),
        //         None => bail!("Could not find any elements with text in the SVG."),
        //     }
        // }

        // if header.is_none() || luck_level.is_none() {
        //     bail!("Either the header field or luck_level field couldn't be found in the SVG.")
        // }

        // let header = header.unwrap();
        // let luck_level = luck_level.unwrap();

        // let mut fortune_slip_svg = FortuneSlipSvg::new(header.clone(), luck_level.clone());
        // for (category, element) in category_to_fortune.into_iter() {
        //     fortune_slip_svg.add_category(category, element.clone());
        // }

        // Ok(fortune_slip_svg)
    }

    /// Errors:
    ///  - NoMatchingKey
    ///  - Woop Woop
    pub fn add_fortunes(&mut self, fortunes: &[FortuneSlip]) -> Result<()> {
        // Convert to SVG elements

        Ok(())
    }

    /// Writes the fortunes to a folder as PDF files.
    /// Since only 4 slips fit on a page, every 4 slips will generate a new PDF file.
    pub fn write(&self, pdf_path: impl AsRef<Path>) -> Result<()> {
        // Step 1: Convert fortunes into SVG element objects

        // Step 2: Write SVG element objects per 4 into a temporary file.

        // Step 3: Merge the PDFs into a single PDF

        // Note that

        todo!()
    }

    fn convert_to_svg_elements(fortunes: &[FortuneSlip]) -> Vec<Element> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;
    use rstest::{fixture, rstest};
    use tempfile::TempDir;

    use crate::{fortune::fortune_slips::FortuneSlip, svg::svg_editor::SvgEditor};

    use super::FortuneWriter;

    struct SampleData {
        category: String,
        fortune: String,
        luck_level_eng: String,
        luck_level_jap: String,
    }

    #[fixture]
    #[once]
    fn sample_data() -> SampleData {
        SampleData {
            category: "general".to_string(),
            fortune: "You will live a prosperous live!".to_string(),
            luck_level_eng: "Great Luck".to_string(),
            luck_level_jap: "大福".to_string(),
        }
    }

    #[fixture]
    fn svg_editor() -> SvgEditor {
        SvgEditor::open("data/fortune_template/omikuji_frontside_template.svg").unwrap()
    }

    fn get_standard_fortune<'a>(sample_data: &'a SampleData) -> FortuneSlip<'a> {
        let mut fortune_categories = HashMap::new();
        fortune_categories.insert(&sample_data.category, &sample_data.fortune);

        FortuneSlip {
            header: &sample_data.luck_level_jap,
            luck_level: &sample_data.luck_level_eng,
            category_to_fortune: fortune_categories,
        }
    }

    #[rstest]
    fn test_constructor__expect_svg_elements(
        sample_data: &SampleData,
        svg_editor: SvgEditor,
    ) -> Result<()> {
        // let fortune_1 = get_standard_fortune(sample_data);
        // let fortune_2 = fortune_1.clone();

        let fortune_writer = FortuneWriter::load(svg_editor)?;

        println!("Woop woop {:#?}", fortune_writer.svg_elems_original);

        // fortune_writer.add_fortunes(&vec![fortune_1, fortune_2])?;

        Ok(())

        // assert_eq!(fortune_writer.svg_fortunes, vec![&fortune_1, &fortune_2])
    }

    #[rstest]
    fn test_write_single_slip__expect_pdf_2_pages(
        sample_data: &SampleData,
        svg_editor: SvgEditor,
    ) -> Result<()> {
        let pdf_path = TempDir::new()?.path().join("temp.pdf");

        let fortune = get_standard_fortune(sample_data);
        let mut fortune_writer = FortuneWriter::load(svg_editor)?;
        fortune_writer.add_fortunes(&[fortune])?;
        fortune_writer.write(&pdf_path)?;

        assert!(pdf_path.exists());

        // Add check for two pages.

        Ok(())
    }

    #[rstest]
    fn test_write_6_slips__expect_pdf_4_pages(
        sample_data: &SampleData,
        svg_editor: SvgEditor,
    ) -> Result<()> {
        let pdf_path = TempDir::new()?.path().join("temp.pdf");
        let fortune_slips = vec![get_standard_fortune(sample_data); 6];

        let mut fortune_writer = FortuneWriter::load(svg_editor)?;
        fortune_writer.add_fortunes(&fortune_slips)?;
        fortune_writer.write(&pdf_path)?;

        assert!(pdf_path.exists());

        // Add check for two pages.

        Ok(())
    }
}
