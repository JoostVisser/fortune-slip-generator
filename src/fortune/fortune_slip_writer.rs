use std::path::Path;

use crate::{
    constants::NR_SLIPS_PER_PAGE,
    fortune::fortune_splitter::FortuneSlipTextRef,
    svg::svg_editor::{text_elem::TextElem, SvgEditor},
};

use anyhow::{anyhow, bail, Result};

use self::svg_keys::{retrieve_svg_keys, SvgKeys};

pub mod svg_keys;

#[derive(Debug, PartialEq, Eq)]
pub struct FortuneSlipWriter {
    svg_editor: SvgEditor,
    svg_keys_all_slips: Vec<SvgKeys>,
}

impl FortuneSlipWriter {
    pub fn new(
        mut svg_editor: SvgEditor,
        fortune_categories: &[String],
    ) -> Result<FortuneSlipWriter> {
        // Ok(slip_writer)

        let svg_keys = retrieve_svg_keys(&svg_editor, fortune_categories)?;

        Self::center_relevant_elems(&mut svg_editor, &svg_keys)?;

        Ok(FortuneSlipWriter {
            svg_keys_all_slips: svg_keys,
            svg_editor,
        })
    }

    fn center_relevant_elems(
        svg_editor: &mut SvgEditor,
        svg_keys_all_slip: &[SvgKeys],
    ) -> Result<()> {
        for svg_keys_slip in svg_keys_all_slip {
            Self::center_relevant_elems_single_slip(svg_editor, svg_keys_slip)?;
        }

        Ok(())
    }

    fn center_relevant_elems_single_slip(
        svg_editor: &mut SvgEditor,
        svg_keys_slip: &SvgKeys,
    ) -> Result<()> {
        let mut all_keys = vec![&svg_keys_slip.header_key, &svg_keys_slip.luck_level_key];
        all_keys.extend(svg_keys_slip.cat_to_fortune_keys.values());

        for key in all_keys {
            let text_elem = svg_editor.get_elem_with_id(key)?;
            if text_elem.text.contains("[center]") {
                let mut new_text_elem = text_elem.clone();
                Self::center_text_elem(&mut new_text_elem);
                svg_editor.update_text_elem_by_id(new_text_elem)?;
            }
        }

        Ok(())
    }

    fn center_text_elem(text_elem: &mut TextElem) {
        text_elem.attr.insert("x".to_string(), "50%".to_string());
        text_elem
            .attr
            .insert("text-anchor".to_string(), "middle".to_string());
    }

    pub fn write_page(&mut self, fortune_texts: &[FortuneSlipTextRef]) -> Result<()> {
        for (idx, fortune_text) in fortune_texts.iter().enumerate() {
            self.write_to_slip(idx, fortune_text)?;

            if idx > NR_SLIPS_PER_PAGE {
                bail!("Too many fortune texts for one page");
            }
        }

        Ok(())
    }

    pub fn write_to_slip(&mut self, idx: usize, fortune_text: &FortuneSlipTextRef) -> Result<()> {
        let svg_keys: &SvgKeys = self
            .svg_keys_all_slips
            .get(idx)
            .ok_or(anyhow::anyhow!("No svg_elem_keys found for idx: {}", idx))?;

        Self::write_to_elem(
            &mut self.svg_editor,
            &svg_keys.header_key,
            fortune_text.header,
        )?;
        Self::write_to_elem(
            &mut self.svg_editor,
            &svg_keys.luck_level_key,
            fortune_text.luck_level,
        )?;

        for (category, text_elem_key) in &svg_keys.cat_to_fortune_keys {
            let fortune_text = fortune_text
                .category_to_fortune
                .get(category)
                .ok_or(anyhow!("No fortune text found for category: {}", category))?;
            Self::write_to_elem(&mut self.svg_editor, text_elem_key, fortune_text)?;
        }
        Ok(())
    }

    pub fn save_to_svg(&self, svg_path: &impl AsRef<Path>) -> Result<()> {
        self.svg_editor.save_to_svg(svg_path)
    }

    fn write_to_elem(svg_editor: &mut SvgEditor, elem_id: &str, fortune: &str) -> Result<()> {
        let mut new_text_elem = svg_editor.get_elem_with_id(elem_id)?.clone();
        new_text_elem.text = fortune.into();
        svg_editor.update_text_elem_by_id(new_text_elem)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};

    use crate::{
        constants::NR_SLIPS_PER_PAGE,
        fortune::{fortune_data::FortuneData, fortune_splitter::FortuneSlipTextRef},
        svg::svg_editor::SvgEditor,
    };

    use super::FortuneSlipWriter;

    #[fixture]
    fn svg_editor() -> SvgEditor {
        let svg_file_path = "test_utils/data/fortune_template/omikuji_frontside_test.svg";
        SvgEditor::open(svg_file_path).unwrap()
    }

    #[fixture]
    fn fortune_data() -> FortuneData {
        FortuneData::open("test_utils/data/fortune_settings.yaml").unwrap()
    }

    #[fixture]
    fn slip_writer() -> FortuneSlipWriter {
        let svg_editor = svg_editor();
        let fortune_data = fortune_data();
        let fortune_categories = fortune_data.get_categories();

        let fc = fortune_categories
            .into_iter()
            .cloned()
            .collect::<Vec<String>>();

        FortuneSlipWriter::new(svg_editor, &fc).unwrap()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FortuneSlipTextOwned {
        header: String,
        luck_level: String,
        category_to_fortune: HashMap<String, String>,
    }

    impl FortuneSlipTextOwned {
        fn to_ref(&self) -> FortuneSlipTextRef {
            FortuneSlipTextRef {
                header: &self.header,
                luck_level: &self.luck_level,
                category_to_fortune: self.category_to_fortune.iter().collect(),
            }
        }
    }

    #[fixture]
    #[once]
    fn fortune_text() -> FortuneSlipTextOwned {
        let mut category_to_fortune = HashMap::new();
        category_to_fortune.insert("general".to_string(), "general_fortune_text".to_string());
        category_to_fortune.insert("health".to_string(), "health_fortune_text".to_string());
        category_to_fortune.insert("love".to_string(), "love_fortune_text".to_string());

        FortuneSlipTextOwned {
            header: "header_text".to_string(),
            luck_level: "luck_level_text".to_string(),
            category_to_fortune,
        }
    }

    #[rstest]
    fn test_new(_slip_writer: FortuneSlipWriter) {}

    #[rstest]
    fn test_nr_fortunes_per_slip(slip_writer: FortuneSlipWriter) {
        assert_eq!(slip_writer.svg_keys_all_slips.len(), NR_SLIPS_PER_PAGE);
    }

    #[rstest]
    fn test_write_slip(
        mut slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) -> Result<()> {
        let fortune_text_ref = fortune_text.to_ref();

        let fortune_text_slip_0 = retrieve_text_for_slip_id(&slip_writer, 0)?;
        let fortune_text_slip_1 = retrieve_text_for_slip_id(&slip_writer, 1)?;
        assert_ne!(fortune_text_slip_0, *fortune_text);
        assert_ne!(fortune_text_slip_1, *fortune_text);

        slip_writer.write_to_slip(0, &fortune_text_ref)?;

        let fortune_text_slip_0 = retrieve_text_for_slip_id(&slip_writer, 0)?;
        let fortune_text_slip_1 = retrieve_text_for_slip_id(&slip_writer, 1)?;
        assert_eq!(fortune_text_slip_0, *fortune_text);
        assert_ne!(fortune_text_slip_1, *fortune_text);

        Ok(())
    }

    #[rstest]
    fn test_write_page_3_slips_success(
        slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) -> Result<()> {
        test_write_page_x_fortunes(slip_writer, fortune_text, NR_SLIPS_PER_PAGE - 1)
    }

    #[rstest]
    fn test_write_page_4_slips_success(
        slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) -> Result<()> {
        test_write_page_x_fortunes(slip_writer, fortune_text, NR_SLIPS_PER_PAGE)
    }

    #[rstest]
    fn test_write_page_5_slips_error(
        slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) {
        assert!(
            test_write_page_x_fortunes(slip_writer, fortune_text, NR_SLIPS_PER_PAGE + 1).is_err()
        )
    }

    fn test_write_page_x_fortunes(
        mut slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
        nr_fortunes: usize,
    ) -> Result<()> {
        let fortune_text_refs = (0..nr_fortunes)
            .map(|_| fortune_text.to_ref())
            .collect::<Vec<_>>();

        for i in 0..NR_SLIPS_PER_PAGE {
            let fortune_text_slip = retrieve_text_for_slip_id(&slip_writer, i)?;
            assert_ne!(fortune_text_slip, *fortune_text);
        }

        slip_writer.write_page(&fortune_text_refs)?;

        for i in 0..nr_fortunes {
            let fortune_text_slip = retrieve_text_for_slip_id(&slip_writer, i)?;
            assert_eq!(fortune_text_slip, *fortune_text);
        }

        Ok(())
    }

    fn retrieve_text_for_slip_id(
        slip_writer: &FortuneSlipWriter,
        slip_idx: usize,
    ) -> Result<FortuneSlipTextOwned> {
        let slip_keys = &slip_writer.svg_keys_all_slips[slip_idx];

        let header_elem = slip_writer
            .svg_editor
            .get_elem_with_id(&slip_keys.header_key)?;

        let luck_level_elem = slip_writer
            .svg_editor
            .get_elem_with_id(&slip_keys.luck_level_key)?;

        let mut category_to_fortune = HashMap::new();
        for (cat_id, cat_text) in &slip_keys.cat_to_fortune_keys {
            let cat_text_elem = slip_writer.svg_editor.get_elem_with_id(cat_text)?;
            category_to_fortune.insert(cat_id.clone(), cat_text_elem.text.clone());
        }

        Ok(FortuneSlipTextOwned {
            header: header_elem.text.clone(),
            luck_level: luck_level_elem.text.clone(),
            category_to_fortune,
        })
    }

    #[rstest]
    fn test_write_slip_not_enough_categories(
        mut slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) {
        let mut fortune_text_2 = fortune_text.clone();
        fortune_text_2.category_to_fortune.remove("love");

        let fortune_text_ref = fortune_text_2.to_ref();

        assert!(slip_writer.write_to_slip(0, &fortune_text_ref).is_err());
    }

    #[rstest]
    fn test_write_slip_check_element_center(
        mut slip_writer: FortuneSlipWriter,
        fortune_text: &FortuneSlipTextOwned,
    ) -> Result<()> {
        let fortune_text_ref = fortune_text.to_ref();

        let first_slip_keys = &slip_writer.svg_keys_all_slips[0];
        let header_elem = slip_writer
            .svg_editor
            .get_elem_with_id(&first_slip_keys.header_key)?;

        assert!(header_elem.text.contains("[center]"));
        assert_eq!(header_elem.attr["x"], "50%");
        assert_eq!(header_elem.attr["text-anchor"], "middle");

        slip_writer.write_to_slip(0, &fortune_text_ref)?;

        let first_slip_keys = &slip_writer.svg_keys_all_slips[0];
        let header_elem = slip_writer
            .svg_editor
            .get_elem_with_id(&first_slip_keys.header_key)?;
        assert!(!header_elem.text.contains("[center]"));
        assert_eq!(header_elem.attr["x"], "50%");
        assert_eq!(header_elem.attr["text-anchor"], "middle");

        Ok(())
    }

    #[rstest]
    fn test_save_to_svg(slip_writer: FortuneSlipWriter) -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path().join("test.svg");

        slip_writer.save_to_svg(&temp_path)?;

        assert!(temp_path.exists());

        Ok(())
    }
}
