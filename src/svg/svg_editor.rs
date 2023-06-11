use self::{text_elem::TextElem, xml_elem_utils::ElemUtils, xml_tree::XmlTree};

use anyhow::{anyhow, Result};
use std::path::Path;
use xmltree::Element;

pub mod text_elem;
mod xml_elem_utils;
mod xml_tree;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SvgEditor {
    xml_tree: XmlTree,
    text_elems_ordered: Vec<TextElem>, // Ordered by index in the xml tree
                                       // This is needed to split the elements into slips.
}

impl SvgEditor {
    pub fn open<P: AsRef<Path>>(svg_file_path: P) -> Result<SvgEditor> {
        let xml_tree = XmlTree::open(svg_file_path)?;
        let elem_keys = xml_tree.get_elems_with_tag("text");

        let text_elems_ordered = elem_keys
            .iter()
            .map(|&x| TextElem::try_from(x))
            .map(|x| x.and_then(|y| Ok(y)))
            .collect::<Result<Vec<_>>>()?;

        Ok(SvgEditor {
            xml_tree,
            text_elems_ordered,
        })
    }

    pub fn get_text_elems_map_ordered(&self) -> Vec<&TextElem> {
        self.text_elems_ordered.iter().collect()
    }

    pub fn update_text_elem_by_id(&mut self, new_text_elem: TextElem) -> Result<()> {
        let text_elem = self.get_elem_with_id_mut(&new_text_elem.id)?;
        *text_elem = new_text_elem;
        let elem_id = text_elem.id.clone();

        self.save_elem_changes_to_xml_tree(&elem_id)?;

        Ok(())
    }

    pub fn get_elem_with_id(&self, text_elem_id: &str) -> Result<&TextElem> {
        self.text_elems_ordered
            .iter()
            .find(|x| x.id == text_elem_id)
            .ok_or(anyhow!(
                "SvgEditor: Could not find element with id '{text_elem_id}'."
            ))
    }

    fn get_elem_with_id_mut(&mut self, text_elem_id: &str) -> Result<&mut TextElem> {
        self.text_elems_ordered
            .iter_mut()
            .find(|x| x.id == text_elem_id)
            .ok_or(anyhow!(
                "SvgEditor: Could not find element with id '{text_elem_id}'."
            ))
    }

    fn save_elem_changes_to_xml_tree(&mut self, elem_id: &str) -> Result<()> {
        let new_xml_elem: Element = self.create_new_xml_elem(elem_id)?;
        self.xml_tree.replace_elem_by_id(new_xml_elem)?;

        Ok(())
    }

    fn create_new_xml_elem(&self, elem_id: &str) -> Result<Element> {
        let text_elem = self.get_elem_with_id(elem_id)?;
        let mut new_xml_elem = self
            .xml_tree
            .get_elem_with_id(elem_id)
            .ok_or(anyhow!(
                "In the XMLTree, could not find an element with id '{elem_id:?}'."
            ))?
            .clone();

        new_xml_elem.attributes = text_elem.attr.clone();
        new_xml_elem.set_inner_text(&text_elem.text)?;

        Ok(new_xml_elem)
    }

    pub fn save_to_svg<P: AsRef<Path>>(&self, svg_path: P) -> Result<()> {
        self.xml_tree.save(svg_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;
    use rstest::{fixture, rstest};
    use tempfile::TempDir;
    use test_utils;

    const SVG_EXAMPLE: &str = r#"
        <svg height='100' width='120'>
            <text id="text1" fill="black" font-size="24">
                <tspan id="tspan1" x='12' y='24'>Mine turtle!</tspan>
            </text>
        </svg>"#;

    #[fixture]
    fn svg_editor() -> SvgEditor {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        SvgEditor::open(&temp_file.path).unwrap()
    }

    #[rstest]
    fn test_get_text_elems_map(svg_editor: SvgEditor) {
        let text_elems_map = svg_editor.get_text_elems_map_ordered();

        assert_eq!(text_elems_map.len(), 1);

        let text_elem = text_elems_map.first().unwrap();

        assert_eq!(text_elem.attr["fill"], "black");
        assert_eq!(text_elem.attr["font-size"], "24");
        assert_eq!(text_elem.text, "Mine turtle!");
    }

    #[rstest]
    fn test_update_test_elem(mut svg_editor: SvgEditor) {
        let text_elem = svg_editor.get_text_elems_map_ordered()[0];
        assert_eq!(text_elem.attr["fill"], "black");

        let mut new_text_elem = text_elem.clone();
        new_text_elem
            .attr
            .insert("fill".to_string(), "red".to_string());
        svg_editor.update_text_elem_by_id(new_text_elem).unwrap();

        let text_elem = svg_editor.get_text_elems_map_ordered()[0];
        assert_eq!(text_elem.attr["fill"], "red");
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
        let text_elem = svg_editor.get_text_elems_map_ordered()[0];
        assert_eq!(text_elem.attr["fill"], "black");

        let mut new_text_elem = text_elem.clone();
        new_text_elem
            .attr
            .insert("fill".to_string(), "red".to_string());
        svg_editor.update_text_elem_by_id(new_text_elem).unwrap();

        let (_dir, path) = create_file_and_save(&mut svg_editor);

        let new_svg_editor = SvgEditor::open(&path).unwrap();
        let new_text_elem = new_svg_editor.get_text_elems_map_ordered()[0];
        assert_eq!(new_text_elem.attr["fill"], "red");
    }
}
