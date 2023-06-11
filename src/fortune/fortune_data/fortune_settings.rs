use std::{path::PathBuf, collections::HashMap};

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FortuneSettings {
    pub luck_levels: HashMap<String, LuckLevelInfo>,
    pub fortune_content_files: Vec<PathBuf>,
    pub template_front: PathBuf,
    pub template_back: PathBuf,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LuckLevelInfo {
    pub jap: String,
    pub eng: String,
}
