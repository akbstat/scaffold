use serde::Deserialize;

use super::item::ConfigItem;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum Kind {
    SDTM,
    ADAM,
    TFL,
}

pub trait ConfigReader {
    /// ## read configution file and return a vector of ConfigItem
    ///
    /// ### Arguments
    ///
    /// @ force: bool - force to return config items even it contains errors, such as length of filename exceeds the limitation
    fn read(&self, force: bool) -> anyhow::Result<Vec<ConfigItem>>;
}
