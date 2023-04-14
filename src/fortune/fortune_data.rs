use std::path::Path;

use anyhow::Result;

use self::{
    fortune_loader::FortuneDataInner,
    fortune_settings::{FortuneSettings},
};

mod fortune_loader;
pub mod fortune_settings;

#[derive(Debug, PartialEq, Eq)]
pub struct FortuneData {
    fortune_data_inner: FortuneDataInner,
}

impl FortuneData {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let fortune_data_inner = fortune_loader::load_fortune_data(path)?;

        Ok(FortuneData { fortune_data_inner })
    }

    pub fn get_settings(&self) -> &FortuneSettings {
        &self.fortune_data_inner.settings
    }

    pub fn get_fortune_text(&self, category: &str, luck_level_key: &str) -> Option<Vec<&String>> {
        Some(
            self.fortune_data_inner
                .fortunes_per_category
                .get(category)?
                .get(luck_level_key)?
                .iter()
                .collect(),
        )
    }

    pub fn get_categories(&self) -> Vec<&String> {
        self.fortune_data_inner
            .fortunes_per_category
            .keys()
            .collect()
    }

    /// Ease of use function: data is already available
    pub fn get_luck_level_keys(&self) -> Vec<&String> {
        self.fortune_data_inner
            .settings
            .luck_levels
            .keys()
            .collect()
    }
}
