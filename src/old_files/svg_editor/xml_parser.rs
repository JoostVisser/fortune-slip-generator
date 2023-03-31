use anyhow::{anyhow, Result};
use std::{fs::File, io::BufReader, path::Path, str::FromStr};
use xml::{
    attribute::OwnedAttribute, name::OwnedName, reader::XmlEvent as ReaderEvent, EventReader,
};

pub fn load_svg<P: AsRef<Path>>(path_to_svg: &P) -> Result<Vec<ReaderEvent>> {
    let reader = open_file(path_to_svg)?;
    let parser = EventReader::new(reader);

    let mut results = vec![];

    for event in parser {
        results.push(event?);
    }

    Ok(results)
}

fn open_file<P: AsRef<Path>>(path_to_svg: &P) -> Result<BufReader<File>> {
    let file = File::open(path_to_svg)?;
    return Ok(BufReader::new(file));
}

pub fn create_owned_attribute(key: &str, value: &str) -> Result<OwnedAttribute> {
    let owned_attribute = OwnedAttribute::new(create_owned_name(key)?, value);

    Ok(owned_attribute)
}

pub fn create_owned_name(name: &str) -> Result<OwnedName> {
    let owned_name = OwnedName::from_str(&name)
        .map_err(|_| anyhow!("Could not create an attribute name from '{}'", &name))?;

    Ok(owned_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SVG_EXAMPLE: &str = "<svg height='100' width='100'>
                                <tspan x='12' y='24'>Cat</tspan>
                               </svg>";

    enum ReaderEventType {
        StartDocument,
        EndDocument,
        StartElement,
        EndElement,
        Whitespace,
        Characters,
    }

    #[test]
    fn test_read_event_order() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);

        let reader_events = load_svg(&temp_file.path).unwrap();
        println!("Reader events: {:#?}", reader_events);

        let reader_event_order = [
            ReaderEventType::StartDocument,
            ReaderEventType::StartElement,
            ReaderEventType::Whitespace,
            ReaderEventType::StartElement,
            ReaderEventType::Characters,
            ReaderEventType::EndElement,
            ReaderEventType::Whitespace,
            ReaderEventType::EndElement,
            ReaderEventType::EndDocument,
        ];

        for (i, actual_event) in reader_events.iter().enumerate() {
            assert_reader_event(actual_event, &reader_event_order[i]);
        }
    }

    fn assert_reader_event(reader_event: &ReaderEvent, reader_event_type: &ReaderEventType) {
        match reader_event_type {
            ReaderEventType::StartDocument => {
                assert!(matches!(reader_event, ReaderEvent::StartDocument { .. }))
            }
            ReaderEventType::EndDocument => {
                assert!(matches!(reader_event, ReaderEvent::EndDocument { .. }))
            }
            ReaderEventType::StartElement => {
                assert!(matches!(reader_event, ReaderEvent::StartElement { .. }))
            }
            ReaderEventType::EndElement => {
                assert!(matches!(reader_event, ReaderEvent::EndElement { .. }))
            }
            ReaderEventType::Whitespace => {
                assert!(matches!(reader_event, ReaderEvent::Whitespace { .. }))
            }
            ReaderEventType::Characters => {
                assert!(matches!(reader_event, ReaderEvent::Characters { .. }))
            }
        }
    }
}
