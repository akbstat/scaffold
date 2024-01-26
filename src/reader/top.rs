use super::reader::ConfigReader;
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::{Path, PathBuf};

const TOP: &str = "top";
const OUTPUT_NAME_COL_INDEX: usize = 4;
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
    fn read(&self) -> anyhow::Result<Vec<(String, bool)>> {
        let mut outputs: Vec<(String, bool)> = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;

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
            outputs.push((output, false));
        }
        Ok(outputs)
    }
}
