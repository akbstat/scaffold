use super::skeleton::{walk, Node, Skeleton, DOCUMENTS};

pub struct DocumentSkeleton {
    nodes: Vec<Node>,
}
impl DocumentSkeleton {
    pub fn new(template: &[u8]) -> anyhow::Result<DocumentSkeleton> {
        Ok(DocumentSkeleton {
            nodes: serde_json::from_slice(template)?,
        })
    }
}

impl Skeleton for DocumentSkeleton {
    fn paths(&self) -> Vec<String> {
        let mut dirs = vec![];
        self.nodes.iter().for_each(|node| {
            walk(DOCUMENTS, node).into_iter().for_each(|dir| {
                dirs.push(dir);
            });
        });
        dirs
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn documents_test() {
        let template =
            fs::read(r"D:\projects\rusty\mobius_kit\.config\document_skeleton.json").unwrap();
        let skeleton: Box<dyn Skeleton> = Box::new(DocumentSkeleton::new(&template).unwrap());
        println!("{:?}", skeleton.paths());
    }
}
