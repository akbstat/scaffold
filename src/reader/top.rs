use crate::Assignment;

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
const SOURCER_COLUMN_INDEX: usize = 9;
const QCER_COLUMN_INDEX: usize = 10;

pub struct TopReader {
    filepath: PathBuf,
}

impl TopReader {
    pub fn new(filepath: &Path) -> TopReader {
        TopReader {
            filepath: filepath.into(),
        }
    }
    pub fn assignement(&self) -> anyhow::Result<Vec<Assignment>> {
        let mut result = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;
        let range = workbook.worksheet_range(TOP)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n < TARGET_ROWS_START_INDEX {
                continue;
            }
            let sourcer = if let Some(data) = row.get(SOURCER_COLUMN_INDEX) {
                data.as_string()
            } else {
                None
            };
            let qcer = if let Some(data) = row.get(QCER_COLUMN_INDEX) {
                data.as_string()
            } else {
                None
            };
            let task = if let Some(data) = row.get(OUTPUT_NAME_COL_INDEX) {
                data.as_string()
            } else {
                None
            };
            if let Some(task) = task {
                if let Some(sourcer) = sourcer {
                    result.push(Assignment {
                        developer: sourcer,
                        task: format!("{}|dev", &task),
                    });
                }
                if let Some(qcer) = qcer {
                    result.push(Assignment {
                        developer: qcer,
                        task: format!("{}|qc", &task),
                    });
                }
            }
        }
        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_assignment_test() -> anyhow::Result<()> {
        let p = Path::new(r"D:\Studies\ak101\202\stats\idmc\utility\top-ak112-303-CSR.xlsx");
        let reader = TopReader::new(p);
        let assignment = reader.assignement()?;
        println!("{:?}", assignment);
        assert!(assignment.len().gt(&0));
        Ok(())
    }
}
