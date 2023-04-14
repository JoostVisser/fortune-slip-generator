use std::path::Path;

use anyhow::Result;

use self::{fortune_data::FortuneData, fortune_slips::FortuneSlipsFactory};

mod fortune_data;
mod fortune_slips;
mod fortune_writer;

// impl FortuneSlip {
//   fn to_svg_element(...)
// }

pub struct FortuneGenerator {
    fortune_data: FortuneData,
}

impl FortuneGenerator {
    pub fn open(path: impl AsRef<Path>) -> Result<FortuneGenerator> {
        let fortune_data = FortuneData::open(path)?;
        Ok(FortuneGenerator { fortune_data })
    }

    pub fn generate_fortunes(&self) -> Result<()>{

        let fortune_factory = FortuneSlipsFactory::new(&self.fortune_data);
        let all_fortunes = fortune_factory.create_slips()?;



        Ok(())
        // let all_fortunes = fortune_slip_creator.create_fortune_slips();
    }
}
