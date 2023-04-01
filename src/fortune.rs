use std::collections::HashMap;

pub mod fortune_loader;
pub mod fortune_writer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FortuneSlip {
    fortune_header: String,
    fortune_luck_level: String,
    fortune_categories: HashMap<String, String>,
}
