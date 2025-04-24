use super::{item::ConfigItem, reader::ConfigReader};
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::{Path, PathBuf};

const CONTENT: &str = "CONTENT";
const DOMAIN_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 6;

pub struct AdamSpecReader {
    filepath: PathBuf,
}

impl AdamSpecReader {
    pub fn new(filepath: &Path) -> AdamSpecReader {
        AdamSpecReader {
            filepath: filepath.into(),
        }
    }
}

impl ConfigReader for AdamSpecReader {
    fn read(&self, _force: bool) -> anyhow::Result<Vec<ConfigItem>> {
        let mut domains: Vec<ConfigItem> = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;
        let supp = false;
        let qc_required = true;

        let range = workbook.worksheet_range(CONTENT)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n < TARGET_ROWS_START_INDEX {
                continue;
            }
            let domain;
            if let Some(e) = row.get(DOMAIN_COL_INDEX) {
                if e.eq(&Empty) {
                    break;
                }
                domain = e.as_string().unwrap();
            } else {
                break;
            }
            domains.push(ConfigItem {
                name: domain.to_lowercase(),
                supp,
                qc_required,
            });
        }
        Ok(domains)
    }
}
