use anyhow::Result;
use xmltree::{Element, XMLNode};

pub trait ElemUtils {
    fn get_inner_text(&self) -> Result<String>;
    fn set_inner_text(&mut self, new_text: &str) -> Result<()>;
}

impl ElemUtils for Element {

    fn get_inner_text(&self) -> Result<String> {
        let text = self
        .children
        .first()
        .and_then(|node| node.as_element())
        .and_then(|node| node.get_text())
        .ok_or(anyhow::anyhow!(
            "No child element found with text for element {:?}.",
            self
        ))?;

        Ok(text.to_string())
    }

    fn set_inner_text(&mut self, new_text: &str) -> Result<()> {
        let first_child = self
            .children
            .first_mut()
            .and_then(|node| node.as_mut_element())
            .ok_or(anyhow::anyhow!("No child element found."))?;

        first_child.children.clear();
        first_child
            .children
            .push(XMLNode::Text(new_text.to_string()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG_EXAMPLE: &str = r#"
        <text fill="black" font-size="24">
            <tspan x='12' y='24'>Mine turtle!</tspan>
        </text>"#;

    #[test]
    fn test_get_inner_text() {
        let elem = Element::parse(SVG_EXAMPLE.as_bytes()).unwrap();
        assert_eq!(elem.get_inner_text().unwrap(), "Mine turtle!");
    }

    #[test]
    fn test_set_inner_text() {
        let mut elem = Element::parse(SVG_EXAMPLE.as_bytes()).unwrap();
        elem.set_inner_text("new text").unwrap();
        assert_eq!(elem.get_inner_text().unwrap(), "new text");
    }
}
