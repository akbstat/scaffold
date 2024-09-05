use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::Kind;

const SDTM_TEMPLATE: &str = "sdtm";
const ADAM_TEMPLATE: &str = "adam";
const TFL_TEMPLATE: &str = "tfls";
const TEMPLATE_EXTENTION: &str = ".sas";

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub role: String,
    pub offical: bool,
}

pub struct VersionManagerParam<'a> {
    pub offical_path: &'a Path,
    pub private_path: &'a Path,
    pub kind: Kind,
}

pub struct VersionManager {
    offical_path: PathBuf,
    private_path: PathBuf,
    kind: Kind,
}

impl VersionManager {
    pub fn new(param: &VersionManagerParam) -> VersionManager {
        let VersionManagerParam {
            offical_path,
            private_path,
            kind,
        } = param;
        VersionManager {
            offical_path: offical_path.into(),
            private_path: private_path.into(),
            kind: kind.to_owned(),
        }
    }

    pub fn list_templates(&self) -> anyhow::Result<Vec<Version>> {
        let mut templates = vec![];
        let mut offical = self.get_offical_template()?;
        let mut private = self.get_private_template()?;
        templates.append(&mut offical);
        templates.append(&mut private);
        Ok(templates)
    }

    pub fn read_template(&self, version: &Version) -> anyhow::Result<Vec<u8>> {
        let Version {
            name,
            role,
            offical,
        } = version;
        let base_directory = self.base_directory(*offical);
        let filename = format!("{}.{}{}", role, name, TEMPLATE_EXTENTION);
        let filepath = base_directory.join(filename);
        Ok(fs::read(&filepath)?)
    }

    pub fn save_template(&self, version: &Version, bytes: &[u8]) -> anyhow::Result<()> {
        let Version {
            name,
            role,
            offical,
        } = version;
        let base_directory = self.base_directory(*offical);
        let filename = format!("{}.{}{}", role, name, TEMPLATE_EXTENTION);
        let filepath = base_directory.join(filename);
        fs::write(filepath, bytes)?;
        Ok(())
    }

    fn base_directory(&self, offical: bool) -> PathBuf {
        let kind = match self.kind {
            Kind::SDTM => SDTM_TEMPLATE,
            Kind::ADAM => ADAM_TEMPLATE,
            Kind::TFL => TFL_TEMPLATE,
        };

        if offical {
            self.offical_path.join(kind)
        } else {
            self.private_path.join(kind)
        }
    }

    fn get_template_list(&self, offical: bool) -> anyhow::Result<Vec<Version>> {
        let mut versions = vec![];
        let directory = self.base_directory(offical);
        if !directory.exists() {
            fs::create_dir_all(&directory)?;
        } else {
            for entry in fs::read_dir(&directory)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    continue;
                }
                let filename = entry.file_name();
                let filename = filename.to_string_lossy();
                if let Some(version) = self.filename_to_version(&filename, offical) {
                    versions.push(version);
                }
            }
        }
        Ok(versions)
    }

    fn get_offical_template(&self) -> anyhow::Result<Vec<Version>> {
        Ok(self.get_template_list(true)?)
    }

    fn get_private_template(&self) -> anyhow::Result<Vec<Version>> {
        Ok(self.get_template_list(false)?)
    }

    fn filename_to_version(&self, filename: &str, offical: bool) -> Option<Version> {
        if !filename.ends_with(TEMPLATE_EXTENTION) {
            None
        } else {
            let filename = filename.split(".").collect::<Vec<&str>>();
            let size = filename.len();
            if size.lt(&3) {
                None
            } else {
                let role = filename[0].to_owned();
                let name = filename[1..size - 1].join(".");
                Some(Version {
                    name,
                    role,
                    offical,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_version_manager() -> anyhow::Result<()> {
        let offical_path = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let private_path = Path::new(r"D:\Users\yuqi01.chen\.temp\app\mobiuskit\scaffold");
        let kind = Kind::SDTM;
        let manager = VersionManager::new(&VersionManagerParam {
            offical_path,
            private_path,
            kind,
        });
        let templates = manager.list_templates()?;
        assert_eq!(5, templates.len());

        let ver = Version {
            name: "v1".into(),
            role: "dev".into(),
            offical: true,
        };

        let tmp = manager.read_template(&ver)?;

        let new_ver = Version {
            name: "v1.2".into(),
            role: "dev".into(),
            offical: false,
        };

        manager.save_template(&new_ver, &tmp)?;

        Ok(())
    }
}
