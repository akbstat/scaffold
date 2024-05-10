use super::{
    errors::{OutputError, OUTPUT_NAME_EXCEED},
    item::ConfigItem,
    reader::ConfigReader,
};
use anyhow::anyhow;
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::{Path, PathBuf};

const TOP: &str = "top";
const OUTPUT_NAME_COL_INDEX: usize = 4;
const VALIDATION_LEVEL_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 1;
const MAX_EMPTY_ROW_COUNT: usize = 10;

pub struct TopReader {
    filepath: PathBuf,
}

impl TopReader {
    pub fn new(filepath: &Path) -> TopReader {
        TopReader {
            filepath: filepath.into(),
        }
    }
}

impl ConfigReader for TopReader {
    fn read(&self) -> anyhow::Result<Vec<ConfigItem>> {
        let mut empty_row_count = 0;
        let mut outputs: Vec<ConfigItem> = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;
        let supp = false;
        let mut qc_required = true;
        let mut error_info = vec![];

        let range = workbook.worksheet_range(TOP)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n < TARGET_ROWS_START_INDEX {
                continue;
            }
            let output;
            if let Some(e) = row.get(OUTPUT_NAME_COL_INDEX) {
                if e.eq(&Empty) {
                    if empty_row_count > MAX_EMPTY_ROW_COUNT {
                        break;
                    } else {
                        empty_row_count += 1;
                        continue;
                    }
                }
                output = e.as_string().unwrap();
            } else {
                break;
            }

            if output.len() > 30 {
                error_info.push(OutputError {
                    item: output.clone(),
                    message: OUTPUT_NAME_EXCEED.into(),
                });
            }

            if let Some(e) = row.get(VALIDATION_LEVEL_COL_INDEX) {
                qc_required = if e.as_string().unwrap().trim().eq("3") {
                    true
                } else {
                    false
                };
            }
            outputs.push(ConfigItem {
                name: output.to_lowercase(),
                supp,
                qc_required,
            });
        }
        if error_info.len() > 0 {
            let error_message = serde_json::to_string(&error_info)?;
            return Err(anyhow!(error_message));
        }
        Ok(outputs)
    }
}
