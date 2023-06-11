use anyhow::{anyhow, Context, Result};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};
use xmltree::Element;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XmlTree {
    root: Element,
}

impl XmlTree {
    pub fn open<P: AsRef<Path>>(svg_file_path: P) -> Result<XmlTree> {
        let svg_file_path = svg_file_path.as_ref();
        let root = Self::read_svg(svg_file_path)
            .with_context(|| format!("Could not read SVG file at path {:?}", svg_file_path))?;

        let xml_tree_wrapper = XmlTree { root };

        Ok(xml_tree_wrapper)
    }

    fn read_svg<P: AsRef<Path>>(svg_file_path: P) -> Result<Element> {
        let file = File::open(svg_file_path.as_ref())?;
        let file = BufReader::new(file);
        Ok(Element::parse(file)?)
    }

    /// Returns all elements whose tag matches given name.
    ///
    /// The tag of an XML element is the identifying string after '<'.
    /// For example `<hello x="3" y="7">Some Text</hello>` returns "hello"
    pub fn get_elems_with_tag<'a>(&'a self, name: &str) -> Vec<&'a Element> {
        Self::get_matching_elems(&self.root, name)
    }

    fn get_matching_elems<'a>(element: &'a Element, name: &str) -> Vec<&'a Element> {
        let mut mut_elems = vec![];

        element
            .children
            .iter()
            .filter_map(|x| x.as_element())
            .for_each(|x| mut_elems.extend(Self::get_matching_elems(x, name)));

        if element.name == name {
            mut_elems.push(element);
        }

        mut_elems
    }

    pub fn get_elem_with_id<'a>(&'a self, elem_id: &str) -> Option<&'a Element> {
        Self::find_matching_elem(&self.root, elem_id)
    }

    fn find_matching_elem<'a>(element: &'a Element, elem_id: &str) -> Option<&'a Element> {
        if let Some(cur_elem_id) = element.attributes.get("id") {
            if cur_elem_id == elem_id {
                return Some(element);
            }
        }

        element
            .children
            .iter()
            .filter_map(|x| x.as_element())
            .find_map(|x| Self::find_matching_elem(x, elem_id))
    }

    pub fn replace_elem_by_id(&mut self, new_elem: Element) -> Result<()> {
        let elem_id = new_elem
            .attributes
            .get("id")
            .ok_or(anyhow!("Element to replace must have an id."))?;
        let found_elem = Self::find_matching_elem_mut(&mut self.root, elem_id)
            .ok_or(anyhow!("Could not find element with id '{:?}'.", elem_id))?;

        *found_elem = new_elem;
        Ok(())
    }

    fn find_matching_elem_mut<'a>(
        element: &'a mut Element,
        elem_id: &str,
    ) -> Option<&'a mut Element> {
        if let Some(cur_elem_id) = element.attributes.get("id") {
            if cur_elem_id == elem_id {
                return Some(element);
            }
        }

        element
            .children
            .iter_mut()
            .filter_map(|x| x.as_mut_element())
            .find_map(|x| Self::find_matching_elem_mut(x, elem_id))
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.root.write(&mut writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::svg::svg_editor::xml_elem_utils::ElemUtils;

    use super::*;
    use rstest::{fixture, rstest};
    use test_utils;

    const SVG_EXAMPLE: &str = r#"
        <svg height='100' width='100'>
            <text id="text1" fill="black" font-size="24">
                <tspan id="tspan1" x='12' y='24'>Mine turtle!</tspan>
            </text>
        </svg>"#;

    #[fixture]
    fn xml_tree() -> XmlTree {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        
        XmlTree::open(temp_file.path).unwrap()
    }

    #[rstest]
    fn test_get_elems_expect_xml_tree_element(xml_tree: XmlTree) {
        let tspan_elems = xml_tree.get_elems_with_tag("tspan");

        assert_eq!(tspan_elems.len(), 1);

        let tspan_elem = &tspan_elems[0];
        assert_eq!(tspan_elem.name, "tspan");
        assert_eq!(tspan_elem.attributes["x"], "12");
        assert_eq!(tspan_elem.attributes["y"], "24");
        assert_eq!(tspan_elem.get_text().unwrap(), "Mine turtle!");
    }

    #[rstest]
    fn test_nested_tag_expect_inner_text(xml_tree: XmlTree) {
        let text_elems = xml_tree.get_elems_with_tag("text");
        let text_elem = &text_elems[0];
        assert_eq!(text_elem.get_inner_text().unwrap(), "Mine turtle!");
    }

    #[rstest]
    fn test_inner_new_text_expect_updated_text(xml_tree: XmlTree) {
        let text_elems = xml_tree.get_elems_with_tag("text");

        let mut text_elem = text_elems[0].clone();
        text_elem.set_inner_text("I like trains").unwrap();

        assert_eq!(text_elem.get_inner_text().unwrap(), "I like trains");
    }

    #[rstest]
    fn test_replace_elem_expect_element_attr_changed(mut xml_tree: XmlTree) {
        let tspan_elem = xml_tree.get_elem_with_id("tspan1").unwrap();

        let mut new_tspan_elem = tspan_elem.clone();

        new_tspan_elem
            .attributes
            .insert("x".to_string(), "69".to_string());

        xml_tree.replace_elem_by_id(new_tspan_elem).unwrap();

        let tspan_elem = &xml_tree.get_elems_with_tag("tspan")[0];
        assert_eq!(tspan_elem.name, "tspan");
        assert_eq!(tspan_elem.attributes["x"], "69");
        assert_eq!(tspan_elem.attributes["y"], "24");
        assert_eq!(tspan_elem.get_text().unwrap(), "Mine turtle!");
    }

    #[rstest]
    fn test_open_target_svg() -> Result<()> {
        let svg_path = "test_utils/data/fortune_template/omikuji_frontside_test.svg";
        let xml_tree = XmlTree::open(svg_path)?;

        let xml_elems = xml_tree.get_elems_with_tag("text");
        assert!(xml_elems.len() > 4);
        assert_eq!(xml_elems.len() % 4, 0);

        Ok(())
    }

    #[rstest]
    fn test_save_expect_updated_text() -> Result<()> {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);
        let mut xml_tree = XmlTree::open(&temp_file.path)?;

        // Step 1: Get the text element
        let text_elem = xml_tree.get_elem_with_id("text1").unwrap();
        assert_eq!(text_elem.get_inner_text().unwrap(), "Mine turtle!");

        // Step 2: Update the text element
        let mut new_elem = text_elem.clone();
        new_elem.set_inner_text("I like trains").unwrap();
        xml_tree.replace_elem_by_id(new_elem)?;

        // Step 3: Save the updated text element
        let save_file = temp_file.dir.path().join("temp2.svg");
        xml_tree.save(&save_file)?;
        assert!(save_file.exists());

        // Step 4: Open the saved file and check the text element
        let xml_tree = XmlTree::open(&save_file)?;
        let text_elems = xml_tree.get_elems_with_tag("text");
        assert_eq!(text_elems[0].get_inner_text().unwrap(), "I like trains");

        Ok(())
    }
}
