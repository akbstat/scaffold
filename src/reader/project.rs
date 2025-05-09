use std::{fs, path::Path};

use regex::Regex;
use serde::Serialize;

const STAT: &str = "stats";

#[derive(Debug, Serialize)]
pub struct Product {
    id: String,
    name: String,
    trials: Vec<Trial>,
}

#[derive(Debug, Serialize)]
pub struct Trial {
    id: String,
    name: String,
    purpose: Vec<Purpose>,
}

#[derive(Debug, Serialize)]
pub struct Purpose {
    id: String,
    name: String,
}

pub fn list_projects(root: &Path) -> anyhow::Result<Vec<Product>> {
    let pattern = Regex::new(r"^ak\d{3}$")?;
    let mut products = vec![];
    let product_list = list_folders(root)?
        .into_iter()
        .filter(|dir| pattern.is_match(dir))
        .collect::<Vec<String>>();
    for product in product_list {
        let product_dir = root.join(&product);
        let mut trials = vec![];
        for trial in list_folders(&product_dir)? {
            let trail_id = format!("{}-{}", &product, &trial);
            let purposes = list_folders(product_dir.join(&trial).join(STAT).as_path())?;
            let trial = Trial {
                id: trail_id.clone(),
                name: trial,
                purpose: purposes
                    .into_iter()
                    .map(|p| Purpose {
                        id: format!("{}-{}", &trail_id, &p),
                        name: p,
                    })
                    .collect::<Vec<Purpose>>(),
            };
            trials.push(trial);
        }
        products.push(Product {
            id: product.clone(),
            name: product,
            trials,
        })
    }
    Ok(products)
}

fn list_folders(root: &Path) -> anyhow::Result<Vec<String>> {
    let mut folders = vec![];
    let root = fs::read_dir(root);
    if root.is_err() {
        return Ok(folders);
    }
    for entry in root?.into_iter() {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            continue;
        }
        folders.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(folders)
}

#[cfg(test)]
mod project_test {
    use super::*;
    #[test]
    fn list_projects_test() {
        let root = Path::new(r"D:\Studies");
        let projects = list_projects(root).unwrap();
        println!("{:?}", projects);
    }
}
