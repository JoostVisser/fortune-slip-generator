use std::collections::HashMap;

use anyhow::bail;
use xml::{attribute::OwnedAttribute, reader::XmlEvent as ReaderXmlEvent};

use super::xml_parser;

#[derive(Debug, PartialEq)]
pub struct BorrowedEventWithId <'a> {
    pub event: &'a ReaderXmlEvent,
    pub id: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SvgEvent {
    TextElement {
        id: usize,
        attributes: HashMap<String, String>,
    },
    Characters {
        id: usize,
        text: String,
    },
}

impl SvgEvent {
    fn can_convert_to_svg_event(xml_event: &ReaderXmlEvent) -> bool {
        let xml_owned_name_start = xml_parser::create_owned_name("text");
        if xml_owned_name_start.is_err() {
            return false;
        }

        let xml_owned_name_start = xml_owned_name_start.unwrap();

        match xml_event {
            ReaderXmlEvent::StartElement { name, .. } if name == &xml_owned_name_start => true,
            ReaderXmlEvent::Characters(_) => true,
            _ => false,
        }
    }

    fn new_start_element(id: usize, attributes: &Vec<OwnedAttribute>) -> Self {
        let attr_map = Self::convert_owned_attr_vector_to_map(attributes);
        SvgEvent::TextElement {
            id,
            attributes: attr_map,
        }
    }

    fn convert_owned_attr_vector_to_map(attr_vec: &Vec<OwnedAttribute>) -> HashMap<String, String> {
        attr_vec
            .iter()
            .map(|attr| (attr.name.local_name.clone(), attr.value.to_string()))
            .collect()
    }

    fn new_characters_element(id: usize, text: &String) -> Self {
        SvgEvent::Characters {
            id,
            text: text.clone(),
        }
    }
}

impl TryFrom<BorrowedEventWithId<'_>> for SvgEvent {
    type Error = anyhow::Error;

    fn try_from(xml_event: BorrowedEventWithId) -> Result<Self, Self::Error> {
        if !Self::can_convert_to_svg_event(&xml_event.event) {
            bail!(
                "Can't convert XmlElement '{:?}' to an SvgElement",
                xml_event
            );
        }

        match &xml_event.event {
            ReaderXmlEvent::StartElement { attributes, .. } => {
                Ok(Self::new_start_element(xml_event.id, attributes))
            }
            ReaderXmlEvent::Characters(text) => {
                Ok(Self::new_characters_element(xml_event.id, text))
            }
            _ => bail!(
                "Can't convert ReaderXmlElement '{:?}' to an SvgElement",
                xml_event
            ),
        }
    }
}
