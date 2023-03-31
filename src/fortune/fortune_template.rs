// use anyhow::{Result, bail};
// use std::path::Path;
// use xml::reader::XmlEvent as ReaderXmlEvent;

// // use crate::svg::svg_editor::{xml_parser, SvgEditor};

// use self::text_elem::SvgTextElem;
// mod text_elem;

// // #[derive(Debug, PartialEq)]
// // pub struct FortuneTemplate<'a> {
// //     svg_editor: SvgEditor<'a>,
// // }

// #[derive(Debug, PartialEq)]
// struct ReaderEventWithIndex<'a> {
//     pub index: usize,
//     pub reader_event: &'a ReaderXmlEvent,
// }

// impl<'a> FortuneTemplate<'a> {
//     // pub fn open<P: AsRef<Path>>(svg_file_path: &P) -> Result<FortuneTemplate> {
//     //     let svg_editor = SvgEditor::open(svg_file_path)?;

//     //     Ok(FortuneTemplate { svg_editor })
//     // }

//     // pub fn get_text_elems(&self) -> Result<Vec<SvgTextElem>> {
//     //     let svg_text_elem: SvgTextElem;
//     //     for svg_event_with_id in self.get_text_events_with_id() {
//     //         let svg_event = svg_event_with_id.reader_event;
//     //         match svg_event {
//     //             ReaderXmlEvent::StartElement { .. } => {
//     //                 svg_text_elem = SvgTextElem::try_from(svg_event_with_id)?;
//     //             }
//     //             ReaderXmlEvent::Characters { .. } => svg_text_elem.add_text_field(svg_event)
//     //             _ => bail!("Noooooo")
//     //         }

//     //         SvgTextElem::new(event_id, attr)
//     //     }
//     //     todo!()
//     // }

//     // fn get_text_events_with_id(&self) -> Vec<ReaderEventWithIndex> {
//     //     self.svg_editor
//     //         .get_svg_events()
//     //         .iter()
//     //         .enumerate()
//     //         .filter(|(_, &event)| Self::is_text_elem_or_char_event(&event))
//     //         .map(|(index, &event)| ReaderEventWithIndex {
//     //             index,
//     //             reader_event: &event,
//     //         })
//     //         .collect()
//     // }

//     // fn is_text_elem_or_char_event(xml_event: &ReaderXmlEvent) -> bool {
//     //     match xml_event {
//     //         ReaderXmlEvent::StartElement { name, .. } => xml_parser::is_text_name(name),
//     //         ReaderXmlEvent::Characters { .. } => true,
//     //         _ => false,
//     //     }
//     // }
// }
