use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use std::error;

#[derive(Debug)]
struct FortuneText<'a> {
    text_elem_start: Option<BytesStart<'a>>,
    tspan_elem_start: Option<BytesStart<'a>>,
    text: Option<String>,
    tspan_elem_end: Option<BytesEnd<'a>>,
    text_elem_end: Option<BytesEnd<'a>>,
}

impl FortuneText<'_> {
    fn new() -> Self {
        Self {
            text_elem_start: None,
            tspan_elem_start: None,
            text: None,
            tspan_elem_end: None,
            text_elem_end: None,
        }
    }
}

pub fn load_fortune_svg_as_xml(path_to_svg: &str) -> Result<String, Box<dyn error::Error>> {
    let mut reader = Reader::from_file(path_to_svg)?;

    let mut fortune_text_elem = FortuneText::new();

    let mut buf = Vec::new();
    let mut check_text = false;
    loop {
        match reader.read_event_into(&mut buf).unwrap() {
            Event::Start(e) => {
                if e.name().as_ref() == b"text" {
                    fortune_text_elem.text_elem_start = Some(e.into_owned());
                } else if e.name().as_ref() == b"tspan" {
                    fortune_text_elem.tspan_elem_start = Some(e.into_owned());
                    check_text = true;
                }
            }
            Event::Text(e) if check_text == true => {
                fortune_text_elem.text = Some(e.unescape().unwrap().into_owned());
                break;
            }
            Event::End(e) => {
                if e.name().as_ref() == b"text" {
                    fortune_text_elem.text_elem_end = Some(e.into_owned());
                } else if e.name().as_ref() == b"tspan" {
                    fortune_text_elem.tspan_elem_end = Some(e.into_owned());
                }
            }
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }

    println!("Fortune text elem {:#?}: ", fortune_text_elem);

    Ok(String::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ok() {
        let result =
            super::load_fortune_svg_as_xml("data/fortune_template/omikuji_frontside_template.svg");
        assert!(result.is_ok());
    }
}
