use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConfigItem {
    pub name: String,
    pub supp: bool,
    pub qc_required: bool,
}
