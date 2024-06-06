use serde::Deserialize;

pub const DOCUMENTS: &str = "documents";
pub const STAT: &str = "stats";

pub trait Skeleton {
    fn paths(&self) -> Vec<String>;
}

#[derive(Debug, Deserialize)]
pub struct Node {
    name: String,
    children: Option<Vec<Node>>,
}

pub fn walk(parent: &str, node: &Node) -> Vec<String> {
    let mut result = vec![];
    let current_dir = if parent.is_empty() {
        node.name.to_owned()
    } else {
        format!(r"{}\{}", parent, node.name)
    };
    match &node.children {
        Some(children) => {
            for child in children {
                let dirs = walk(&current_dir, child);
                dirs.into_iter().for_each(|dir| result.push(dir));
            }
        }
        None => result.push(current_dir),
    }
    result
}
