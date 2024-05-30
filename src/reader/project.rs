use std::{fs, path::Path};

use regex::Regex;

#[derive(Debug)]
pub struct Product {
    name: String,
    trails: Vec<String>,
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
        let trails = list_folders(&product_dir)?;
        products.push(Product {
            name: product,
            trails,
        })
    }
    Ok(products)
}

fn list_folders(root: &Path) -> anyhow::Result<Vec<String>> {
    let mut folders = vec![];
    let root = fs::read_dir(root)?;
    for entry in root.into_iter() {
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
