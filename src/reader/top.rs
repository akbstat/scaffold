use super::{item::ConfigItem, reader::ConfigReader};
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::{Path, PathBuf};

const TOP: &str = "top";
const OUTPUT_NAME_COL_INDEX: usize = 4;
const VALIDATION_LEVEL_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 1;

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
        let mut outputs: Vec<ConfigItem> = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;
        let supp = false;
        let mut qc_required = true;

        let range = workbook.worksheet_range(TOP)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n < TARGET_ROWS_START_INDEX {
                continue;
            }
            let output;
            if let Some(e) = row.get(OUTPUT_NAME_COL_INDEX) {
                if e.eq(&Empty) {
                    break;
                }
                output = e.as_string().unwrap();
            } else {
                break;
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
        Ok(outputs)
    }
}
