use anyhow::{Ok, Result};
use xml::reader::XmlEvent as ReaderXmlEvent;

use self::{
    svg_event::{BorrowedEventWithId, SvgEvent},
    xml_parser::load_svg,
};
use std::path::Path;
mod serde_test;
pub mod svg_event;
pub mod xml_parser;

#[derive(Debug, PartialEq)]
pub struct SvgEditor<'a> {
    svg_file_path: &'a Path,
    svg_events: Vec<ReaderXmlEvent>,
}

impl<'a> SvgEditor<'a> {
    pub fn open<P: AsRef<Path>>(svg_file_path: &'a P) -> Result<SvgEditor<'a>> {
        let svg_events = load_svg(svg_file_path)?;

        let svg_editor = SvgEditor {
            svg_file_path: svg_file_path.as_ref(),
            svg_events,
        };

        Ok(svg_editor)
    }

    pub fn get_svg_events(&self) -> Vec<SvgEvent> {
        self.svg_events
            .iter()
            .enumerate()
            .filter_map(|(id, xml_event)| Self::xml_event_to_svg_event(id, xml_event))
            .collect()
    }

    fn xml_event_to_svg_event(id: usize, xml_event: &ReaderXmlEvent) -> Option<SvgEvent> {
        let borred_event = BorrowedEventWithId {
            id,
            event: xml_event,
        };
        let svg_event = SvgEvent::try_from(borred_event);
        svg_event.ok()
    }

    pub fn update_svg_event(id: usize, svg_event: SvgEvent) {
        todo!()
    }

    // pub fn change_attr(
    //     &mut self,
    //     elem_id: usize,
    //     new_attr: &HashMap<String, String>,
    // ) -> Result<()> {
    //     let svg_event = &mut self.svg_events[elem_id];

    //     println!("Svg event's attr before: {:#?}", svg_event);

    //     if let ReaderXmlEvent::StartElement { attributes, .. } = svg_event {
    //         Self::change_all_attr(attributes, new_attr)?;
    //         Ok(())
    //     } else {
    //         bail!(
    //             "Event of elem_id ({}) is of type '{:?}', \
    //            but should be of type 'ReaderXmlEvent::StartElement'",
    //             elem_id,
    //             svg_event
    //         )
    //     }
    // }

    // pub fn update_attr(
    //     old_attr: &mut Vec<OwnedAttribute>,
    //     new_entry: (&String, &String),
    // ) -> Result<()> {
    //     let owned_key = xml_parser::create_owned_name(new_entry.0)?;

    //     let mut attributes_with_matched_keys = old_attr
    //         .iter_mut()
    //         .filter(|x| x.name == owned_key)
    //         .collect::<Vec<_>>();

    //     match attributes_with_matched_keys.len() {
    //         1 => {
    //             attributes_with_matched_keys[0].value = new_entry.1.to_string();
    //             Ok(())
    //         }
    //         0 => bail!(
    //             "Could not find any attribute with key '{}', instead found {:?}",
    //             new_entry.0,
    //             old_attr
    //         ),
    //         i => bail!(
    //             "Found irregular keys number of keys ({i}),
    //             this should not be possible. Attributes: {:?}",
    //             old_attr
    //         ),
    //     }
    // }

    // fn change_all_attr(
    //     old_attr: &mut Vec<OwnedAttribute>,
    //     new_attr: &HashMap<String, String>,
    // ) -> Result<()> {
    //     old_attr.retain(|x| false);
    //     for (key, value) in new_attr.iter() {
    //         let owned_attr = xml_parser::create_owned_attribute(key, value)?;
    //         old_attr.push(owned_attr);
    //     }

    //     Ok(())
    // }

    pub fn save_to_svg<P: AsRef<Path>>(&self, output_svg_path: P) {
        todo!()
    }

    // pub fn save_to_pdf<P: AsRef<Path>>(&self, output_pdf_path: P) {
    //     svg_to_pdf(self.svg_file_path, output_pdf_path.as_ref());
    // }
}
