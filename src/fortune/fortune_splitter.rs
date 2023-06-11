use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};

use super::{fortune_data::fortune_settings::FortuneSettings, FortuneData};

#[derive(Debug, PartialEq, Eq)]
pub struct FortuneSlipTextRef<'a> {
    pub header: &'a str,
    pub luck_level: &'a str,
    pub category_to_fortune: HashMap<&'a String, &'a String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FortuneSplitter<'a> {
    fortune_data: &'a FortuneData,
    fortune_settings: &'a FortuneSettings,
}

impl<'a> FortuneSplitter<'a> {
    pub fn new(fortune_data: &'a FortuneData) -> Self {
        FortuneSplitter {
            fortune_data,
            fortune_settings: fortune_data.get_settings(),
        }
    }

    /// Creates and returns a randomized vector of fortune slips.
    ///
    /// The fortune slips themselves are groups of references for each slip.
    ///
    /// The factory creates as many slips as possible without duplication by finding
    /// for every luck level the topic with the least fortunes.
    ///
    pub fn shuffle_and_split(&self) -> Result<Vec<FortuneSlipTextRef<'a>>> {
        let luck_level_keys = self.fortune_data.get_luck_level_keys();

        let mut all_slips = vec![];

        for key in luck_level_keys {
            let slips = self.create_random_slips_for_luck(key)?;
            all_slips.extend(slips);
        }

        // Not necessary, but output PDFs are more pleasing this way.
        all_slips.sort_by_key(|key| key.luck_level);

        Ok(all_slips)
    }

    fn create_random_slips_for_luck(
        &self,
        luck_level_key: &String,
    ) -> Result<Vec<FortuneSlipTextRef<'a>>> {
        let slips_per_category = self.get_shuffled_fortunes_per_category(luck_level_key)?;

        let nr_fortunes = self.max_nr_of_fortunes(&slips_per_category)?;

        (0..nr_fortunes)
            .into_iter()
            .map(|idx| self.get_slip_for_idx(idx, &slips_per_category, &luck_level_key))
            .collect()
    }

    fn get_shuffled_fortunes_per_category(
        &self,
        luck_level_key: &String,
    ) -> Result<HashMap<&'a String, Vec<&'a String>>> {
        let luck_categories = self.fortune_data.get_categories();
        let mut category_to_slips = HashMap::new();

        for &luck_category in &luck_categories {
            let fortune_slips = self.get_fortunes_shuffled(luck_category, luck_level_key)?;
            category_to_slips.insert(luck_category, fortune_slips);
        }

        Ok(category_to_slips)
    }

    fn get_fortunes_shuffled(
        &self,
        luck_category: &String,
        luck_level_key: &String,
    ) -> Result<Vec<&'a String>> {
        let mut fortune_slips = self
            .fortune_data
            .get_fortune_text(&luck_category, &luck_level_key)
            .ok_or(anyhow!(
                "Could not find luck category {} with key {} in the fortune data.",
                luck_category,
                luck_level_key
            ))?;
        fortune_slips.shuffle(&mut thread_rng());

        Ok(fortune_slips)
    }

    fn max_nr_of_fortunes(
        &self,
        slips_per_category: &HashMap<&String, Vec<&String>>,
    ) -> Result<usize> {
        slips_per_category
            .values()
            .map(|x| x.len())
            .min()
            .ok_or(anyhow!(
                "Gathered slips are empty: {:?}.",
                slips_per_category
            ))
    }

    fn get_slip_for_idx(
        &self,
        idx: usize,
        slips_per_category: &HashMap<&'a String, Vec<&'a String>>,
        luck_level_key: &String,
    ) -> Result<FortuneSlipTextRef<'a>> {
        let luck_level_info = self
            .fortune_settings
            .luck_levels
            .get(luck_level_key)
            .ok_or(anyhow!("Key not found in luck levels: {}.", luck_level_key))?;

        let cat_to_fort = self.get_fortune_text_for_single_slip(slips_per_category, idx)?;

        Ok(FortuneSlipTextRef {
            header: &luck_level_info.jap,
            luck_level: &luck_level_info.eng,
            category_to_fortune: cat_to_fort,
        })
    }

    fn get_fortune_text_for_single_slip(
        &self,
        slips_per_category: &HashMap<&'a String, Vec<&'a String>>,
        idx: usize,
    ) -> Result<HashMap<&'a String, &'a String>> {
        let mut category_to_one_fortune = HashMap::new();
        for cat in self.fortune_data.get_categories() {
            let fortune = slips_per_category
                .get(cat)
                .and_then(|x| x.get(idx))
                .ok_or(anyhow!("Error"))?;

            category_to_one_fortune.insert(cat, *fortune);
        }

        Ok(category_to_one_fortune)
    }
}

#[cfg(test)]
mod tests {
    use crate::fortune::{fortune_data::FortuneData, fortune_splitter::FortuneSplitter};

    #[test]
    fn test_create_fortune_slips() -> anyhow::Result<()> {
        let fortune_data = FortuneData::open("test_utils/data/fortune_settings.yaml")?;

        let fortune_slip_creator = FortuneSplitter::new(&fortune_data);
        let fortune_slips = fortune_slip_creator.shuffle_and_split()?;

        assert_eq!(fortune_slips.len(), 5);
        assert_eq!(
            fortune_slips
                .iter()
                .filter(|x| x.luck_level == "Good Luck")
                .count(),
            3
        );
        assert_eq!(
            fortune_slips
                .iter()
                .filter(|x| x.luck_level == "Bad Luck")
                .count(),
            2
        );

        Ok(())
    }
}
