use crate::reader::{item::ConfigItem, new_reader, Kind};
use crate::render::{Item, Render};
use anyhow::Ok;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub custom_code: Vec<String>,
    pub path: String,
    pub template: String,
}

pub struct Generator {
    items: Vec<ConfigItem>,
    template: Render,
    kind: Kind,
    assignment: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResult {
    name: String,
    existed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub developer: String,
    pub task: String,
}

impl Generator {
    pub fn new(
        config: &Path,
        kind: Kind,
        assignment: Vec<Assignment>,
        force: bool,
    ) -> anyhow::Result<Generator> {
        let items = new_reader(&kind, config).read(force)?;
        let template = Render::new()?;
        let assignment = if assignment.len() > 0 {
            let mut assign_map = HashMap::new();
            assignment.iter().for_each(|assign| {
                assign_map.insert(assign.task.to_string(), assign.developer.to_string());
            });
            Some(assign_map)
        } else {
            None
        };
        Ok(Generator {
            items,
            template,
            kind,
            assignment,
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
            let developer = if let Some(assignments) = &self.assignment {
                let task = if Group::Qc.eq(&param.group) {
                    format!("{}|qc", name)
                } else {
                    format!("{}|dev", name)
                };
                if let Some(developer) = assignments.get(&task) {
                    format!("{:27}", developer.to_string())
                } else {
                    format!("{:27}", " ")
                }
            } else {
                format!("{:27}", " ")
            };
            let item = Item {
                name: name.into(),
                study: param.study.clone(),
                engine: param.engine.clone(),
                purpose: purpose(&name, &param.group, &self.kind)?,
                start: current.clone(),
                description: "Create".into(),
                supp: *supp,
                developer,
                slot: param.custom_code.clone(),
                path: param.path.clone(),
            };
            let filename = filename(name, &param.group);
            let existed = self
                .template
                .render(&param.template, &item, &dest.join(&filename))?;
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
        let dev_template = fs::read_to_string(Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\code\template\sdtm\dev.v1.sas",
        ))
        .unwrap();
        let qc_template = fs::read_to_string(Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\code\template\sdtm\qc.v1.sas",
        ))
        .unwrap();
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: vec!["%format".into(), "%checklog".into(), "".into()],
            template: dev_template,
            path: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: vec!["%format".into(), "%checklog".into(), "".into()],
            template: qc_template,
            path: "".into(),
        };
        let config = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 SDTM Specification v0.2.xlsx",
        );
        let dev_dest = Path::new(r"D:\Studies\ak112\303\stats\CSR\product\program\sdtm");
        let qc_dest = Path::new(r"D:\Studies\ak112\303\stats\CSR\validation\program\sdtm");
        let g = Generator::new(config, Kind::SDTM, vec![], false).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
    #[test]
    fn adam_generate_test() {
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: vec!["%format".into(), "%checklog".into()],
            template: "".into(),
            path: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: vec!["%format".into(), "%checklog".into()],
            template: "".into(),
            path: "".into(),
        };
        let config = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let dev_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\adam\dev");
        let qc_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\adam\qc");
        let g = Generator::new(config, Kind::ADAM, vec![], false).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
    #[test]
    fn tfl_generate_test() {
        let dev = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Dev,
            custom_code: vec!["".into()],
            template: "".into(),
            path: "".into(),
        };
        let qc = Param {
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            group: Group::Qc,
            custom_code: vec!["".into()],
            template: "".into(),
            path: "".into(),
        };
        let config = Path::new(r"D:\Studies\ak112\303\stats\CSR\utility\top-ak112-303-CSR.xlsx");
        let dev_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\tfl\dev");
        let qc_dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\tfl\qc");
        let g = Generator::new(config, Kind::TFL, vec![], false).unwrap();
        g.render(dev_dest, &dev).unwrap();
        g.render(qc_dest, &qc).unwrap();
    }
}
