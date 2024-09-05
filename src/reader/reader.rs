use serde::Deserialize;

use super::item::ConfigItem;

#[derive(Debug, Clone, Deserialize)]
pub enum Kind {
    SDTM,
    ADAM,
    TFL,
}

pub trait ConfigReader {
    fn read(&self) -> anyhow::Result<Vec<ConfigItem>>;
}
