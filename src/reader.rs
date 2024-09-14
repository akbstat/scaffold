use std::path::Path;

use crate::Assignment;

use self::{adam_spec::AdamSpecReader, sdtm_spec::SdtmSpecReader, top::TopReader};

mod adam_spec;
mod errors;
pub mod item;
mod project;
mod reader;
mod sdtm_spec;
pub mod top;

pub use self::item::ConfigItem;
pub use self::project::list_projects;
pub use self::reader::Kind;

pub fn new_reader(kind: &Kind, filepath: &Path) -> Box<dyn reader::ConfigReader> {
    match kind {
        Kind::SDTM => Box::new(SdtmSpecReader::new(filepath)),
        Kind::ADAM => Box::new(AdamSpecReader::new(filepath)),
        Kind::TFL => Box::new(TopReader::new(filepath)),
    }
}

pub fn read_assignment_from_top(filepath: &Path) -> anyhow::Result<Vec<Assignment>> {
    let reader = TopReader::new(filepath);
    Ok(reader.assignement()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_sdtm_spec_test() {
        let config = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 SDTM Specification v0.2.xlsx",
        );
        let reader = new_reader(&Kind::SDTM, config);
        let result = reader.read().unwrap();
        assert_eq!(result.len(), 37);
    }
    #[test]
    fn read_adam_spec_test() {
        let config = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let reader = new_reader(&Kind::ADAM, config);
        let result = reader.read().unwrap();
        assert_eq!(result.len(), 17);
    }
    #[test]
    fn read_tfl_spec_test() {
        let config = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\specs\top-ak112-303-CSR.xlsx");
        let reader = new_reader(&Kind::TFL, config);
        let result = reader.read().unwrap();
        assert_eq!(result.len(), 144);
    }
}
