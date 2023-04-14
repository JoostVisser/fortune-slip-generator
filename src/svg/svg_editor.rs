use self::{text_elem::TextElem, xml_elem_utils::ElemUtils, xml_tree_wrapper::XmlTreeWrapper};

use super::svg_to_pdf;
use anyhow::Result;
use std::path::Path;
use tempfile::TempDir;
use xmltree::Element;

pub mod text_elem;
mod xml_elem_utils;
mod xml_tree_wrapper;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SvgEditor {
    xml_tree: XmlTreeWrapper,
    text_elems: Vec<TextElem>,
    xml_elems: Vec<Element>,
}

impl SvgEditor {
    pub fn open<P: AsRef<Path>>(svg_file_path: P) -> Result<SvgEditor> {
        let xml_tree = XmlTreeWrapper::open(svg_file_path)?;
        let xml_elems = xml_tree.get_elems_with_tag("text");
        let text_elems = xml_elems
            .iter()
            .map(|x| TextElem::try_from(x))
            .collect::<Result<_>>()?;

        Ok(SvgEditor {
            xml_tree,
            text_elems,
            xml_elems,
        })
    }

    pub fn get_text_elems(&self) -> Vec<&TextElem> {
        return self.text_elems.iter().collect();
    }

    pub fn get_text_elems_mut(&mut self) -> Vec<&mut TextElem> {
        return self.text_elems.iter_mut().collect();
    }

    pub fn save_to_pdf<P: AsRef<Path>>(&mut self, pdf_path: P) -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_svg_path = temp_dir.path().join("temp.svg");

        self.save_to_svg(&temp_svg_path)?;
        svg_to_pdf::svg_to_pdf(&temp_svg_path, &pdf_path)?;

        Ok(())
    }

    pub fn save_to_svg<P: AsRef<Path>>(&mut self, svg_path: P) -> Result<()> {
        self.save_elem_changes()?;
        self.xml_tree.save(svg_path)?;
        Ok(())
    }

    fn save_elem_changes(&mut self) -> Result<()> {
        for (text_elem, xml_elem) in self.text_elems.iter().zip(&self.xml_elems) {
            let new_xml_elem = Self::clone_new_xml_elem(&text_elem, &xml_elem)?;
            self.xml_tree.replace_elem(xml_elem, new_xml_elem)?;
        }

        Ok(())
    }

    fn clone_new_xml_elem(text_elem: &TextElem, xml_elem: &Element) -> Result<Element> {
        let mut new_xml_elem = xml_elem.clone();
        new_xml_elem.attributes = text_elem.attr.clone();
        new_xml_elem.set_inner_text(&text_elem.text)?;
        Ok(new_xml_elem)
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;
    use rstest::{fixture, rstest};
    use test_utils;

    const SVG_EXAMPLE: &str = r#"
        <svg height='100' width='120'>
            <text fill="black" font-size="24">
                <tspan x='12' y='24'>Mine turtle!</tspan>
            </text>
        </svg>"#;

    #[fixture]
    fn svg_editor() -> SvgEditor {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        SvgEditor::open(&temp_file.path).unwrap()
    }

    #[rstest]
    fn test_get_text_elems(svg_editor: SvgEditor) {
        let text_elems = svg_editor.get_text_elems();

        assert_eq!(text_elems.len(), 1);

        let text_elem = text_elems.first().unwrap();

        assert_eq!(text_elem.attr["fill"], "black");
        assert_eq!(text_elem.attr["font-size"], "24");
        assert_eq!(text_elem.text, "Mine turtle!");
    }

    #[rstest]
    fn test_get_text_elems_mut(mut svg_editor: SvgEditor) {
        let mut text_elems = svg_editor.get_text_elems_mut();

        let text_elem = text_elems.first_mut().unwrap();
        assert_eq!(text_elem.attr["fill"], "black");

        text_elem.attr.insert("fill".to_string(), "red".to_string());
        assert_eq!(text_elem.attr["fill"], "red");
    }

    #[rstest]
    fn test_save_to_pdf(mut svg_editor: SvgEditor) {
        let temp_dir = TempDir::new().unwrap();
        let pdf_path = temp_dir.path().join("temp.pdf");

        svg_editor.save_to_pdf(&pdf_path).unwrap();
        assert!(pdf_path.exists());
    }

    #[rstest]
    fn test_save_to_svg(mut svg_editor: SvgEditor) {
        let (_dir, svg_path) = create_file_and_save(&mut svg_editor);
        assert!(svg_path.exists());
    }

    fn create_file_and_save(svg_editor: &mut SvgEditor) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let svg_path = temp_dir.path().join("temp2.svg");
        svg_editor.save_to_svg(&svg_path).unwrap();
        (temp_dir, svg_path)
    }

    #[rstest]
    fn test_save_to_svg_with_changes(mut svg_editor: SvgEditor) {
        let mut text_elems = svg_editor.get_text_elems_mut();
        let text_elem = text_elems.first_mut().unwrap();
        assert_eq!(text_elem.attr["fill"], "black");
        text_elem.attr.insert("fill".to_string(), "red".to_string());

        let (_dir, path) = create_file_and_save(&mut svg_editor);

        let new_svg_editor = SvgEditor::open(&path).unwrap();
        let new_text_elems = new_svg_editor.get_text_elems();
        let new_text_elem = new_text_elems.first().unwrap();
        assert_eq!(new_text_elem.attr["fill"], "red");
    }
}
