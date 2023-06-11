use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::Path,
};

use super::fortune_settings::FortuneSettings;

type FortunesPerCategory = HashMap<String, LuckToFortunes>;
type LuckToFortunes = HashMap<String, Vec<String>>;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct FortuneDataInner {
    pub settings: FortuneSettings,
    pub fortunes_per_category: FortunesPerCategory,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct FortuneTextDataFromFile {
    #[serde(rename = "fortune_key")]
    category: String,
    fortunes: LuckToFortunes,
}

pub fn load_fortune_data(path: impl AsRef<Path>) -> Result<FortuneDataInner> {
    let fortune_settings = load_fortune_settings_with_updated_path(&path)?;

    let fortunes_per_category = load_fortune_contents(&fortune_settings.fortune_content_files)?;

    let fortune_data = FortuneDataInner {
        settings: fortune_settings,
        fortunes_per_category,
    };

    error_check(&fortune_data)?;

    Ok(fortune_data)
}

fn load_fortune_settings_with_updated_path(path: impl AsRef<Path>) -> Result<FortuneSettings> {
    let reader = open_file_with_context(path.as_ref())?;
    let mut fortune_settings: FortuneSettings = serde_yaml::from_reader(reader)?;

    update_settings_paths(path.as_ref(), &mut fortune_settings)?;

    Ok(fortune_settings)
}

fn open_file_with_context(path: impl AsRef<Path>) -> Result<BufReader<File>> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Could not read from file at {:?}", path.as_ref()))?;

    Ok(BufReader::new(file))
}

fn update_settings_paths(
    path: impl AsRef<Path>,
    fortune_settings: &mut FortuneSettings,
) -> Result<()> {
    let parent_path = path
        .as_ref()
        .parent()
        .ok_or(anyhow!("Could not find the parent of {:?}", path.as_ref()))?;

    for path in &mut fortune_settings.fortune_content_files {
        *path = parent_path.join(&path);
    }
    fortune_settings.template_back = parent_path.join(&fortune_settings.template_back);
    fortune_settings.template_front = parent_path.join(&fortune_settings.template_front);
    Ok(())
}

fn load_fortune_contents(fortune_paths: &[impl AsRef<Path>]) -> Result<FortunesPerCategory> {
    fortune_paths.iter().map(open_and_flatten).collect()
}

fn open_and_flatten(fortune_path: impl AsRef<Path>) -> Result<(String, LuckToFortunes)> {
    let reader = open_file_with_context(fortune_path)?;
    let content: FortuneTextDataFromFile = serde_yaml::from_reader(reader)?;
    Ok((content.category, content.fortunes))
}

fn error_check(fortune_data: &FortuneDataInner) -> Result<()> {
    check_consistency_luck_levels(fortune_data)?;
    check_unique_keys_categories(fortune_data)?;
    Ok(())
}

fn check_consistency_luck_levels(fortune_data: &FortuneDataInner) -> Result<()> {
    let luck_keys: HashSet<_> = fortune_data.settings.luck_levels.keys().collect();

    for (category, luck_to_fortunes) in &fortune_data.fortunes_per_category {
        let fortune_content_luck_levels: HashSet<_> = luck_to_fortunes.keys().collect();
        if !(luck_keys == fortune_content_luck_levels) {
            anyhow::bail!(
                "Luck levels from category '{}' do not equal the ones from settings.
                 Left: {:?}
                 Right: {:?}",
                category,
                fortune_content_luck_levels,
                luck_keys
            )
        }
    }

    Ok(())
}

fn check_unique_keys_categories(fortune_data: &FortuneDataInner) -> Result<()> {
    let fortunes_per_category = &fortune_data.fortunes_per_category;

    let luck_keys: HashSet<_> = fortunes_per_category.keys().collect();

    if luck_keys.len() != fortunes_per_category.len() {
        anyhow::bail!(
            "The fortune text category keys (e.g. health) \
                       aren't unique across files"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use maplit::hashmap;
    use pretty_assertions::assert_eq;

    use crate::fortune::fortune_data::fortune_settings::{FortuneSettings, LuckLevelInfo};

    use super::{load_fortune_data, load_fortune_settings_with_updated_path};

    #[test]
    fn test_load_fortune_settings_with_updated_path() -> Result<()> {
        let fortune_settings: FortuneSettings =
            load_fortune_settings_with_updated_path("test_utils/data/fortune_settings.yaml")?;

        let expected_settings = get_test_settings();

        assert_eq!(fortune_settings, expected_settings);

        Ok(())
    }

    fn get_test_settings() -> FortuneSettings {
        let luck_levels = hashmap! {
            "good_luck".to_string() => LuckLevelInfo {
                jap: "中吉".to_string(),
                eng: "Good Luck".to_string(),
            },
            "bad_luck".to_string() => LuckLevelInfo {
                jap: "凶".to_string(),
                eng: "Bad Luck".to_string(),
            },
        };

        let template_front =
            PathBuf::from("test_utils/data/fortune_template/omikuji_frontside_test.svg");
        let template_back =
            PathBuf::from("test_utils/data/fortune_template/omikuji_backside_long.svg");

        let fortune_content_files = vec![
            PathBuf::from("test_utils/data/fortune_text/general_fortunes.yaml"),
            PathBuf::from("test_utils/data/fortune_text/health_fortunes.yaml"),
            PathBuf::from("test_utils/data/fortune_text/love_fortunes.yaml"),
        ];

        FortuneSettings {
            luck_levels,
            fortune_content_files,
            template_front,
            template_back,
        }
    }

    #[test]
    fn test_load_fortune_data() -> Result<()> {
        let fortune_data = load_fortune_data("test_utils/data/fortune_settings.yaml")?;

        let love_fortunes = fortune_data.fortunes_per_category.get("love").unwrap();
        let love_good_fortunes = love_fortunes.get("good_luck").unwrap();
        let love_bad_fortunes = love_fortunes.get("bad_luck").unwrap();

        assert_eq!(
            love_good_fortunes,
            &vec![
                "Good mood will attract people.".to_string(),
                "Confess your love and you will be successful.".to_string(),
                "Be open to meet new people, you will find love in the future.".to_string(),
            ]
        );

        assert_eq!(
            love_bad_fortunes,
            &vec![
                "Confess your love and you will be unsuccessful.".to_string(),
                "Don't let a second someone creep into your heart.".to_string(),
                "Wipe your tears and move on, that's the only way you can find new love."
                    .to_string(),
            ]
        );

        Ok(())
    }
}
