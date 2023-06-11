use std::collections::HashMap;

use crate::{
    svg::svg_editor::{text_elem::TextElem, SvgEditor}, constants::NR_SLIPS_PER_PAGE,
};

use anyhow::{bail, Result};

const MANDATORY_TAGS: [&str; 2] = ["header", "luck_level"];

/// Struct that holds the keys for the text elements in the svg file.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SvgKeys {
    pub header_key: String,
    pub luck_level_key: String,
    pub cat_to_fortune_keys: HashMap<String, String>,
}

pub fn retrieve_svg_keys(
    svg_editor: &SvgEditor,
    fortune_categories: &[String],
) -> Result<Vec<SvgKeys>> {
    let text_elems = svg_editor.get_text_elems_map_ordered();

    let elems_per_slip = text_elems.len() / NR_SLIPS_PER_PAGE;

    text_elems
        .chunks_exact(elems_per_slip)
        .map(|chunk| retrieve_svg_elem_keys_for_chunk(chunk, fortune_categories))
        .collect()
}

fn retrieve_svg_elem_keys_for_chunk(
    text_elem_chunk: &[&TextElem],
    fortune_categories: &[String],
) -> Result<SvgKeys> {
    let missing_tags = check_missing_tags(text_elem_chunk, fortune_categories);

    missing_tags?;

    let mut svg_elem_keys = SvgKeys {
        header_key: "".to_string(),
        luck_level_key: "".to_string(),
        cat_to_fortune_keys: HashMap::new(),
    };

    text_elem_chunk
        .iter()
        .filter(|x| is_relevant_text_elem(x, fortune_categories))
        .for_each(|text_elem| add_text_elem_to_keys(&mut svg_elem_keys, text_elem));

    no_empty_keys(&svg_elem_keys, fortune_categories)?;
    Ok(svg_elem_keys)
}

fn check_missing_tags(text_elem_chunk: &[&TextElem], fortune_categories: &[String]) -> Result<()> {
    let mut allowed_tags: Vec<_> = MANDATORY_TAGS.into_iter().map(|x| x.to_string()).collect();
    allowed_tags.extend_from_slice(fortune_categories);

    for text_elem in text_elem_chunk {
        if text_elem.text.contains('_') && !is_relevant_text_elem(text_elem, fortune_categories) {
            bail!(
                "Missing tag in fortune slip: {}. Allowed tags: {:?}",
                text_elem.text,
                allowed_tags
            )
        }
    }

    Ok(())
}

fn is_relevant_text_elem(text_elem: &TextElem, fortune_categories: &[String]) -> bool {
    let mut allowed_tags: Vec<_> = MANDATORY_TAGS.into_iter().map(|x| x.to_string()).collect();
    allowed_tags.extend_from_slice(fortune_categories);

    let result = allowed_tags.iter().any(|x| text_elem.text.contains(x));

    result

}

fn add_text_elem_to_keys(svg_elem_keys: &mut SvgKeys, text_elem: &TextElem) {
    match &text_elem.text {
        s if s.contains("header") => svg_elem_keys.header_key = text_elem.id.clone(),
        s if s.contains("luck_level") => svg_elem_keys.luck_level_key = text_elem.id.clone(),
        s if s.contains('_') => {
            let (category, _) = s.split_once('_').unwrap();
            svg_elem_keys
                .cat_to_fortune_keys
                .insert(category.to_string(), text_elem.id.clone());
        }
        _ => unreachable!("Should not be possible to get here"),
    }
}

fn no_empty_keys(svg_elem_keys: &SvgKeys, fortune_categories: &[String]) -> Result<()> {
    let mut allowed_tags: Vec<_> = MANDATORY_TAGS.into_iter().map(|x| x.to_string()).collect();
    allowed_tags.extend_from_slice(fortune_categories);

    if svg_elem_keys.header_key.is_empty() {
        bail!("Missing header in fortune slip");
    }

    if svg_elem_keys.luck_level_key.is_empty() {
        bail!("Missing luck_level in fortune slip");
    }

    for category in fortune_categories {
        if svg_elem_keys.cat_to_fortune_keys.get(category).is_none() {
            bail!("Missing fortune category in fortune slip: {}", category);
        }
    }

    Ok(())
}
