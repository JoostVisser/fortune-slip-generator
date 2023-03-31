use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use anyhow::Result;
use minidom::{
    element::{Attrs, Texts},
    Element,
};
use tempfile::TempDir;

use super::svg_to_pdf;

#[derive(Debug, PartialEq)]
pub struct SvgEditor {
    pub svg_root: Element,
}

impl SvgEditor {
    pub fn open<P: AsRef<Path>>(svg_file_path: P) -> Result<SvgEditor> {
        let svg_root = Self::read_svg(svg_file_path)?;

        let svg_editor = SvgEditor { svg_root };

        Ok(svg_editor)
    }

    fn read_svg<P: AsRef<Path>>(svg_file_path: P) -> Result<Element> {
        let file = File::open(svg_file_path.as_ref())?;
        let reader = BufReader::new(file);
        Element::from_reader_with_prefixes(reader, "http://www.w3.org/2000/svg".to_string())
            .map_err(|x| anyhow::anyhow!("Could not parse the SVG due to {x}"))
    }

    /// Returns all elements whose name matches given name.
    ///
    /// The name of an XML element is the text inside a tag.
    /// For example `<hello>Some Text</hello>` returns "hello"
    pub fn get_elems_with_name(&self, name: &str) -> Vec<Element> {
        Self::get_matching_elems(&self.svg_root, name)
    }

    fn get_matching_elems<'a>(element: &'a Element, name: &str) -> Vec<Element> {
        let mut mut_elems = vec![];

        for child_elem in element.children() {
            mut_elems.extend(Self::get_matching_elems(child_elem, name))
        }

        if element.name() == name {
            mut_elems.push(element.clone());
        }

        mut_elems
    }

    /// Replace an element for a new element.
    /// This is the main way of interacting with the SVG editor.
    pub fn replace_elem(&mut self, old_elem: &Element, new_elem: &Element) -> Result<()> {
        let found_elem = Self::find_matching_elem(&mut self.svg_root, old_elem)?;
        Self::update_single_element(found_elem, new_elem);
        Ok(())
    }

    fn find_matching_elem<'a>(
        element: &'a mut Element,
        old_elem: &Element,
    ) -> Result<&'a mut Element> {
        if element == old_elem {
            return Ok(element);
        }

        for child_elem in element.children_mut() {
            let matching_elem = Self::find_matching_elem(child_elem, old_elem);

            if let Ok(elem) = matching_elem {
                return Ok(elem);
            }
        }

        anyhow::bail!("No matching element found")
    }

    fn update_single_element(element: &mut Element, new_elem: &Element) {
        Self::update_attrs(element, new_elem.attrs());
        Self::update_text(element, new_elem.texts());
    }

    fn update_attrs(element: &mut Element, new_attrs: Attrs) {
        for (key, value) in new_attrs {
            element.set_attr(key, value)
        }
    }

    fn update_text(element: &mut Element, new_texts: Texts) {
        // Note: If new_texts is larger than the text of the element, it won't append it.
        for (old_text, new_text) in element.texts_mut().zip(new_texts) {
            *old_text = new_text.to_string();
        }
    }

    pub fn save_to_svg<P: AsRef<Path>>(&self, svg_path: P) -> Result<()> {
        let file = File::create(svg_path)?;
        let mut writer = BufWriter::new(file);
        self.svg_root.write_to(&mut writer)?;

        Ok(())
    }

    pub fn save_to_pdf<P: AsRef<Path>>(&self, pdf_path: P) -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_svg_path = temp_dir.path().join("temp.svg");

        self.save_to_svg(&temp_svg_path)?;
        svg_to_pdf::svg_to_pdf(&temp_svg_path, &pdf_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_utils;

    const SVG_EXAMPLE: &str = r#"<svg height='100' width='100' xmlns="svg">
                                <tspan x='12' y='24'>Cat</tspan>
                               </svg>"#;

    #[test]
    fn test_get_elems() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let svg_editor = SvgEditor::open(&temp_file.path).unwrap();

        let tspan_elems = svg_editor.get_elems_with_name("tspan");

        assert_eq!(tspan_elems.len(), 1);

        let tspan_elem = &tspan_elems[0];
        assert_eq!(tspan_elem.name(), "tspan");
        assert_eq!(tspan_elem.attr("x").unwrap(), "12");
        assert_eq!(tspan_elem.attr("y").unwrap(), "24");
        assert_eq!(tspan_elem.text(), "Cat");
    }

    #[test]
    fn test_replace_elem() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let mut svg_editor = SvgEditor::open(&temp_file.path).unwrap();

        let tspan_elem = &svg_editor.get_elems_with_name("tspan")[0];
        let mut new_tspan_elem = tspan_elem.clone();
        new_tspan_elem.set_attr("x", 69);
        svg_editor
            .replace_elem(&tspan_elem, &new_tspan_elem)
            .unwrap();

        let tspan_elem = &svg_editor.get_elems_with_name("tspan")[0];
        assert_eq!(tspan_elem.name(), "tspan");
        assert_eq!(tspan_elem.attr("x").unwrap(), "69");
        assert_eq!(tspan_elem.attr("y").unwrap(), "24");
        assert_eq!(tspan_elem.text(), "Cat");
    }

    const SVG_EXAMPLE_CIRCLE: &str = r#"<svg height="100" width="100">
        <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        Sorry, your browser does not support inline SVG.
    </svg> "#;

    #[test]
    fn test_save_svg() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE_CIRCLE);
        let svg_editor = SvgEditor::open(&temp_file.path).unwrap();

        let svg_path = temp_file.dir.path().join("test1.svg");
        svg_editor.save_to_svg(&svg_path).unwrap();
        assert!(svg_path.exists());
    }

    #[test]
    fn test_save_pdf() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE_CIRCLE);
        let svg_editor = SvgEditor::open(&temp_file.path).unwrap();

        let pdf_path = temp_file.dir.path().join("test1.pdf");
        svg_editor.save_to_pdf(&pdf_path).unwrap();
        assert!(pdf_path.exists())
    }
}
