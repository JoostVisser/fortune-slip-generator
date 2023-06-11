use std::collections::HashMap;

use anyhow::bail;
use xmltree::Element;

use super::xml_elem_utils::ElemUtils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextElem {
    pub id: String,
    pub attr: HashMap<String, String>,
    pub text: String,
}

impl TryFrom<&Element> for TextElem {
    type Error = anyhow::Error;

    fn try_from(value: &Element) -> Result<Self, Self::Error> {
        if value.name != "text" {
            bail!("Element is not a text element");
        }

        if !value.attributes.contains_key("id") {
            bail!("Text element '{}' has no id", value.get_inner_text()?);
        }

        return Ok(TextElem {
            id: value.attributes["id"].to_string(),
            attr: value.attributes.clone(),
            text: value.get_inner_text()?.to_string(),
        });
    }
}
