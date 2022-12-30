use serde::{Serialize, Deserialize};
use std::{error::Error, fs, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
struct Fortune {
    fortune_type: String,
    fortunes: HashMap<String, Vec<String>>
}

pub fn read_yaml_fortunes() -> Result<(), Box<dyn Error>> {
    read_yaml_fortune("data/fortune_text/general_fortunes.yaml")
}

fn read_yaml_fortune(path: &str) -> Result<(), Box<dyn Error>>  {
    let rdr = fs::File::open(path)?;

    let data: Fortune = serde_yaml::from_reader(rdr)?;

    println!("Fortune type: {}", data.fortune_type);
    println!("All fortunes: {:#?}", data.fortunes);

    Ok(())
}
