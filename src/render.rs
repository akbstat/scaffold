use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub study: String,
    pub engine: String,
    pub purpose: String,
    pub start: String,
    pub description: String,
    pub supp: bool,
    pub developer: String,
    pub slot: Vec<String>,
}

pub struct Render {
    template: Tera,
}

impl Render {
    pub fn new(mut templates: &Path) -> anyhow::Result<Render> {
        if templates.is_file() {
            templates = &templates.parent().unwrap();
        }
        let template = Tera::new(&format!(
            "{}/*.sas",
            templates.to_string_lossy().to_string()
        ))?;
        Ok(Render { template })
    }
    /// if file already existed before created, return true, else return false
    pub fn render(&self, template: &str, item: &Item, dest: &Path) -> anyhow::Result<bool> {
        let mut ctx = Context::new();
        ctx.insert("item", item);
        let mut data = self
            .template
            .render(&format!("{}.sas", template), &ctx)?
            .as_bytes()
            .to_vec();
        // add BOM
        data.insert(0, 239);
        data.insert(1, 187);
        data.insert(2, 191);
        let file_existed = dest.exists();
        if !file_existed {
            fs::write(dest, data)?;
        }
        Ok(file_existed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Item;
    #[test]
    fn sdtm_template_test() {
        let templates = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let sdtm = Render::new(templates).unwrap();
        let item = Item {
            name: "lb".into(),
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            purpose: "SDTM.LB".into(),
            start: "14MAR2023".into(),
            description: "Create".into(),
            supp: true,
            developer: "yuki".into(),
            slot: vec!["test1".into(), "test2".into()],
        };
        let dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\lb.sas");
        sdtm.render("sdtm/dev.v1", &item, dest).unwrap();
        let dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\v-lb.sas");
        sdtm.render("sdtm/qc.v1", &item, dest).unwrap();
    }

    #[test]
    fn adam_template_test() {
        let templates = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let sdtm = Render::new(templates).unwrap();
        let item = Item {
            name: "adsl".into(),
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            purpose: "ADAM.ADSL".into(),
            start: "14MAR2023".into(),
            description: "Create".into(),
            supp: true,
            developer: "yuki".into(),
            slot: vec![],
        };
        let dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\adsl.sas");
        sdtm.render("adam/dev.v1", &item, dest).unwrap();
        let dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\v-adsl.sas");
        sdtm.render("adam/qc.v1", &item, dest).unwrap();
    }

    #[test]
    fn tfl_template_test() {
        let templates = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\template");
        let sdtm = Render::new(templates).unwrap();
        let item = Item {
            name: "l-16-02-07-06-irae-ss".into(),
            study: "AK112-303".into(),
            engine: "SAS EG".into(),
            purpose: "xxxx".into(),
            start: "14MAR2023".into(),
            description: "Create".into(),
            supp: true,
            developer: "yuki".into(),
            slot: vec![],
        };
        let dest = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\l-16-02-07-06-irae-ss.sas");
        sdtm.render("tfls/dev.v1", &item, dest).unwrap();
        let dest =
            Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\v-l-16-02-07-06-irae-ss.sas");
        sdtm.render("tfls/qc.v1", &item, dest).unwrap();
    }
}
