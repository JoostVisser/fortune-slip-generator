use std::collections::HashMap;

use xmltree::Element;

use super::xml_elem_utils::ElemUtils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextElem {
    pub attr: HashMap<String, String>,
    pub text: String,
}

impl TryFrom<&Element> for TextElem {
    type Error = anyhow::Error;

    fn try_from(value: &Element) -> Result<Self, Self::Error> {
        return Ok(TextElem {
            attr: value.attributes.clone(),
            text: value.get_inner_text()?.to_string(),
        });
    }
}

impl TextElem {
    fn center_text(&mut self) -> &Self {
        self.attr.insert("x".to_string(), "50%".to_string());
        self.attr
            .insert("text-anchor".to_string(), "middle".to_string());
        return self;
    }
}
