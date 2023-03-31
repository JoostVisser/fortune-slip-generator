use std::collections::HashMap;

pub mod fortune_loader;
pub mod fortune_template;


pub struct FortuneSlip {
    fortune_header: FortuneEntry,
    fortune_luck_level: FortuneEntry,
    fortune_categories: HashMap<String, FortuneEntry>
}

pub struct FortuneEntry {
    entry_type: FortuneEntryType,
    entry_text: String
}

pub enum FortuneEntryType {
    FortuneHeader,
    FortuneLuckLevel,
    FortuneLuckCategory(String),
}
