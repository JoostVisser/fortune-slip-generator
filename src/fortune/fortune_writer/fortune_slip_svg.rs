use xmltree::Element;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FortuneSlipSvg {
    header: Element,
    luck_level: Element,
    category_to_fortune: HashMap<String, Element>,
}

impl FortuneSlipSvg {
    pub fn new(header: Element, luck_level: Element) -> Self {
        FortuneSlipSvg {
            header,
            luck_level,
            category_to_fortune: HashMap::new()
        }
    }

    pub fn add_category(&mut self, category: String, svg_elem: Element) {
        self.category_to_fortune.insert(category, svg_elem);
    }

    pub fn center_elements(&mut self) {}

    pub fn header_text_to(&mut self, new_text: String) {}

    pub fn luck_level_text_to(&mut self, new_text: String) {}

    pub fn category_text_to(&mut self, new_text: String) {}
}

#[cfg(test)]
mod tests {
    use minidom::Element;

    fn test_fortune_slip_svg_creation() {
        let elem = Element::builder("name", "namespace")
            .attr("name", "value")
            .append("inner")
            .build();

        assert_eq!(elem.name(), "name");
        assert_eq!(elem.ns(), "namespace".to_owned());
        assert_eq!(elem.attr("name"), Some("value"));
        assert_eq!(elem.attr("inexistent"), None);
        assert_eq!(elem.text(), "inner");
    }
}
