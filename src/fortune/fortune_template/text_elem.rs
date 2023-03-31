use std::collections::HashMap;

use anyhow::{bail, Result};
use xml::{attribute::OwnedAttribute, reader::XmlEvent as ReaderXmlEvent};

use crate::svg::svg_editor::xml_parser;

use super::ReaderEventWithIndex;

#[derive(Debug, PartialEq, Eq)]
pub struct SvgTextElem {
    pub event_id: usize,
    attr: HashMap<String, String>,
    inner_text_fields: Vec<TextWithId>,
}

impl SvgTextElem {
    pub fn new(event_id: usize, attr: HashMap<String, String>) -> Self {
        SvgTextElem {
            event_id,
            attr,
            inner_text_fields: Vec::new(),
        }
    }

    pub fn owned_attr_vector_to_map(attr_vec: &Vec<OwnedAttribute>) -> HashMap<String, String> {
        attr_vec
            .iter()
            .map(|attr| (attr.name.local_name.clone(), attr.value.to_string()))
            .collect()
    }

    pub fn center_text(&mut self) -> &Self {
        self.attr.insert("x".to_string(), "50%".to_string());
        self.attr
            .insert("text-anchor".to_string(), "middle".to_string());
        return self;
    }

    pub fn add_text_field(&mut self, text_field: TextWithId) {
        self.inner_text_fields.push(text_field);
    }

    pub fn get_entries(&self) -> &Vec<TextWithId> {
        &self.inner_text_fields
    }
}

impl TryFrom<ReaderEventWithIndex<'_>> for SvgTextElem {
    type Error = anyhow::Error;

    fn try_from(xml_event_with_id: ReaderEventWithIndex) -> Result<Self> {
        match xml_event_with_id.reader_event {
            ReaderXmlEvent::StartElement {
                // name: xml_parser::TEXT_OWNED_NAME,
                attributes,
                ..
            } => {
                let attr_map = SvgTextElem::owned_attr_vector_to_map(attributes);
                let new_svg_text_elem = SvgTextElem::new(xml_event_with_id.index, attr_map);
                Ok(new_svg_text_elem)
            }

            _ => bail!("Could not convert due to wrong event type:"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TextWithId {
    pub event_id: usize,
    pub text: String,
}
