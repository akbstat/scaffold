use super::{item::ConfigItem, reader::ConfigReader};
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::{Path, PathBuf};

const CONTENT: &str = "CONTENT";
const SUPP_PREFIX: &str = "SUPP";
const DOMAIN_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 6;
const VAR_BELONG_COL_INDEX: usize = 9;

pub struct SdtmSpecReader {
    filepath: PathBuf,
}

impl SdtmSpecReader {
    pub fn new(filepath: &Path) -> SdtmSpecReader {
        SdtmSpecReader {
            filepath: filepath.into(),
        }
    }
}

impl ConfigReader for SdtmSpecReader {
    fn read(&self) -> anyhow::Result<Vec<ConfigItem>> {
        let mut domains: Vec<ConfigItem> = vec![];
        let mut workbook: Xlsx<_> = open_workbook(self.filepath.as_path())?;
        let qc_required = true;

        let range = workbook.worksheet_range(CONTENT)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n < TARGET_ROWS_START_INDEX {
                continue;
            }
            let domain;
            let mut supp = false;
            if let Some(e) = row.get(DOMAIN_COL_INDEX) {
                if e.eq(&Empty) {
                    break;
                }
                domain = e.as_string().unwrap();
            } else {
                break;
            }
            if skip_supp(&domain) {
                continue;
            }
            // read domain detail sheet to find out if supp existed
            if let Ok(range) = workbook.worksheet_range(&domain) {
                for row in range.rows().into_iter().rev() {
                    if let Some(cell) = row.get(VAR_BELONG_COL_INDEX) {
                        if cell.eq(&Empty) {
                            continue;
                        } else {
                            if cell.as_string().unwrap().eq(SUPP_PREFIX) {
                                supp = true;
                                break;
                            }
                        }
                    }
                }
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

/// if read supp domain in content sheet, just skip,
/// because will determine existence of supp in details
/// of main domain
fn skip_supp(domain: &str) -> bool {
    domain.starts_with(SUPP_PREFIX)
}
