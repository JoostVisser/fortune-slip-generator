use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
struct Fortune {
    fortune_type: String,
    fortunes: HashMap<String, Vec<String>>
}

pub fn read_yaml_fortunes() -> Result<()> {
    read_yaml_fortune("data/fortune_text/general_fortunes.yaml")
}

fn read_yaml_fortune(path: &str) -> Result<()>  {
    let rdr = fs::File::open(path)?;

    let data: Fortune = serde_yaml::from_reader(rdr)?;

    println!("Fortune type: {}", data.fortune_type);
    println!("All fortunes: {:#?}", data.fortunes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read_yaml_fortune;


    #[test]
    fn test_read_yaml_fortune() {
        let fortune_path = "data/fortune_text/general_fortunes.yaml";
        read_yaml_fortune(fortune_path).unwrap();

    }
}
