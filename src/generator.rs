use crate::reader::{item::ConfigItem, new_reader, Kind};
use crate::render::{Item, Render};
use anyhow::Ok;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Group {
    Dev,
    Qc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    pub study: String,
    pub engine: String,
    pub group: Group,
    pub custom_code: String,
}

pub struct Generator {
    items: Vec<ConfigItem>,
    template: Render,
    kind: Kind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResult {
    name: String,
    existed: bool,
}

impl Generator {
    pub fn new(config: &Path, template_dir: &Path, kind: Kind) -> anyhow::Result<Generator> {
        let items = new_reader(&kind, config).read()?;
        let template = Render::new(template_dir)?;
        Ok(Generator {
            items,
            template,
            kind,
        })
    }
    pub fn render(&self, mut dest: &Path, param: &Param) -> anyhow::Result<Vec<FileResult>> {
        let mut result = vec![];
        if dest.is_file() {
            dest = dest.parent().unwrap();
        }
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }
        let current = Local::now().format("%e%b%Y").to_string().to_uppercase();
        for ConfigItem {
            name,
            supp,
            qc_required,
        } in &self.items
        {
            if (!qc_required) && Group::Qc.eq(&param.group) {
                continue;
            }
            let item = Item {
                name: name.into(),
                study: param.study.clone(),
                engine: param.engine.clone(),
                purpose: purpose(&name, &param.group, &self.kind)?,
                start: current.clone(),
                description: "Create".into(),
                supp: *supp,
                developer: "    ".into(),
                slot: vec![param.custom_code.clone()],
            };
            let template = format!(
                "{}/{}",
                match self.kind {
                    Kind::SDTM => "sdtm",
                    Kind::ADAM => "adam",
                    Kind::TFL => "tfls",
                },
                group_template("v1", &param.group)
            );
            let filename = filename(name, &param.group);
            let existed = self
                .template
                .render(&template, &item, &dest.join(&filename))?;
            result.push(FileResult {
                name: filename,
                existed,
            })
        }
        Ok(result)
    }
}

fn filename(item: &str, group: &Group) -> String {
    match group {
        Group::Dev => format!("{}.sas", item),
        Group::Qc => format!("v-{}.sas", item),
    }
}

fn group_template(version: &str, group: &Group) -> String {
    match group {
        Group::Dev => format!("dev.{}", version),
        Group::Qc => format!("qc.{}", version),
    }
}

fn purpose(item: &str, group: &Group, kind: &Kind) -> anyhow::Result<String> {
    let action = match group {
        Group::Dev => "To Create",
        Group::Qc => "To Qc",
    };
    let output = match kind {
        Kind::SDTM => format!("SDTM.{} dataset", item.to_uppercase()),
        Kind::ADAM => format!("ADAM.{} dataset", item.to_uppercase()),
        Kind::TFL => {
            let output_type = if item.starts_with("t") {
                "table"
            } else if item.starts_with("f") {
                "figure"
            } else {
                "listing"
            };
            let name = String::from_utf8(item.as_bytes().get(2..).unwrap().to_vec())?;
            format!("{} {}", output_type, name.replace("-", "."))
        }
    };
    Ok(format!("{} {}", action, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sdtm_generate_test() {
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: "".into(),
        };
        let config = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 SDTM Specification v0.2.xlsx",
        );
        let template_dir = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let dev_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\sdtm\dev");
        let qc_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\sdtm\qc");
        let g = Generator::new(config, template_dir, Kind::SDTM).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
    #[test]
    fn adam_generate_test() {
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: "".into(),
        };
        let config = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let template_dir = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let dev_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\adam\dev");
        let qc_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\adam\qc");
        let g = Generator::new(config, template_dir, Kind::ADAM).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
    #[test]
    fn tfl_generate_test() {
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: "".into(),
        };
        let config = Path::new(r"D:\Studies\ak112\303\stats\CSR\utility\top-ak112-303-CSR.xlsx");
        let template_dir = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let dev_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\tfl\dev");
        let qc_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\tfl\qc");
        let g = Generator::new(config, template_dir, Kind::TFL).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
}
