use std::path::Path;

use anyhow::Result;

use self::{fortune_loader::FortuneDataInner, fortune_settings::FortuneSettings};

mod fortune_loader;
pub mod fortune_settings;

#[derive(Debug, PartialEq, Eq)]
pub struct FortuneData {
    fortune_data_inner: FortuneDataInner,
}

impl FortuneData {
    /// Opens the fortune data from the given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let fortune_data_inner: FortuneDataInner = fortune_loader::load_fortune_data(path)?;

        Ok(FortuneData { fortune_data_inner })
    }

    /// Returns the settings of the fortune data.
    ///
    /// Example:
    /// ```
    /// # use anyhow::Ok;
    /// use fortune_generator::fortune_data::FortuneData;
    ///
    /// let fortune_data = FortuneData::open("test_utils/data/fortune_settings.yaml")
    ///     .unwrap();
    /// let settings = fortune_data.get_settings();
    ///
    /// assert_eq!(settings.luck_levels.len(), 2);
    /// # Ok(())
    /// ```
    pub fn get_settings(&self) -> &FortuneSettings {
        &self.fortune_data_inner.settings
    }

    /// Returns the fortune texts for the given category and luck level.
    ///
    /// Example:
    /// ```
    /// # use anyhow::Ok;
    /// use fortune_generator::fortune_data::FortuneData;
    ///
    /// let fortune_data = FortuneData::open("test_utils/data/fortune_settings.yaml")?;
    /// let fortune_texts = fortune_data.get_fortune_text("health", "bad_luck").unwrap();
    ///
    /// assert_eq!(fortune_texts[0], "The weather will cause sickness to spread around.");
    /// assert_eq!(fortune_texts[1], "Energy levels will be low for the coming week.");
    /// # Ok(())
    /// ```
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

    /// Returns a reference to the categories of the fortune data.
    ///
    /// Example:
    /// ```
    /// # use anyhow::Ok;
    /// use fortune_generator::fortune_data::FortuneData;
    ///
    /// let fortune_data = FortuneData::open("test_utils/data/fortune_settings.yaml")?;
    /// let categories = fortune_data.get_categories();
    ///
    /// assert!(categories.contains(&&"health".to_string()));
    /// # Ok(())
    /// ```
    pub fn get_categories(&self) -> Vec<&String> {
        self.fortune_data_inner
            .fortunes_per_category
            .keys()
            .collect()
    }

    /// Returns a reference to the luck level keys of the fortune data.
    ///
    /// Example:
    /// ```
    /// # use anyhow::Ok;
    /// use fortune_generator::fortune_data::FortuneData;
    ///
    /// let fortune_data = FortuneData::open("test_utils/data/fortune_settings.yaml")?;
    /// let luck_level_keys = fortune_data.get_luck_level_keys();
    ///
    /// assert!(luck_level_keys.contains(&&"bad_luck".to_string()));
    /// # Ok(())
    /// ```
    pub fn get_luck_level_keys(&self) -> Vec<&String> {
        self.fortune_data_inner
            .settings
            .luck_levels
            .keys()
            .collect()
    }
}
