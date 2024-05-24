use serde::{Deserialize, Serialize};

pub const OUTPUT_NAME_EXCEED: &str = "Length of output name exceed 30";

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputError {
    pub item: String,
    pub message: String,
}
