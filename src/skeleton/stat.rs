use super::skeleton::{walk, Node, Skeleton, STAT};

pub struct StatSkeleton {
    purpose: String,
    nodes: Vec<Node>,
}
impl StatSkeleton {
    pub fn new(purpose: &str, template: &[u8]) -> anyhow::Result<StatSkeleton> {
        Ok(StatSkeleton {
            purpose: purpose.into(),
            nodes: serde_json::from_slice(template)?,
        })
    }
}

impl Skeleton for StatSkeleton {
    fn paths(&self) -> Vec<String> {
        let mut dirs = vec![];
        self.nodes.iter().for_each(|node| {
            walk(&format!(r"{}\{}", STAT, self.purpose), node)
                .into_iter()
                .for_each(|dir| {
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
    fn stat_test() {
        let template =
            fs::read(r"D:\projects\rusty\mobius_kit\.config\stat_skeleton.json").unwrap();
        let skeleton: Box<dyn Skeleton> = Box::new(StatSkeleton::new("CSR", &template).unwrap());
        println!("{:?}", skeleton.paths());
    }
}
