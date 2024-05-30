use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
};

use super::skeleton::Skeleton;

#[derive(Debug, Default)]
pub struct Builder {
    root: PathBuf,
    product_id: RefCell<String>,
    trial_id: RefCell<String>,
}

impl Builder {
    pub fn new(root: &Path) -> Builder {
        Builder {
            root: root.to_path_buf(),
            ..Default::default()
        }
    }
    pub fn set_product_id(&self, product_id: &str) -> &Self {
        *self.product_id.borrow_mut() = product_id.into();
        self
    }
    pub fn set_trial_id(&self, trial_id: &str) -> &Self {
        *self.trial_id.borrow_mut() = trial_id.into();
        self
    }
    pub fn build(&self, skeleton: impl Skeleton) -> anyhow::Result<()> {
        let dirs = skeleton.paths();
        for dir in dirs {
            let p = self
                .root
                .join(self.product_id.borrow().as_str())
                .join(self.trial_id.borrow().as_str())
                .join(dir);
            fs::create_dir_all(p)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod skeleton_test {

    use std::fs;

    use crate::skeleton::{documents::DocumentSkeleton, stat::StatSkeleton};

    use super::*;
    #[test]
    fn test_generate_skeleton() {
        let root = Path::new(r"D:\Studies");
        let product_id = "ak101";
        let trial_id = "202";
        let purpose = "CSR";

        let document_template =
            fs::read(r"D:\projects\rusty\mobius_kit\.config\document_skeleton.json").unwrap();
        let stat_template =
            fs::read(r"D:\projects\rusty\mobius_kit\.config\stat_skeleton.json").unwrap();
        let document_skeleton = DocumentSkeleton::new(&document_template).unwrap();
        let stat_skeleton = StatSkeleton::new(purpose, &stat_template).unwrap();

        let builder = Builder::new(root);
        builder.set_product_id(product_id).set_trial_id(trial_id);
        builder.build(document_skeleton).unwrap();
        builder.build(stat_skeleton).unwrap();
        assert!(Path::new("D:\\Studies\\ak101\\202\\documents\\specs").exists());
        assert!(Path::new("D:\\Studies\\ak101\\202\\documents\\protocol").exists());
        assert!(Path::new("D:\\Studies\\ak101\\202\\stat\\CSR\\product\\dataset\\sdtm").exists());
        assert!(Path::new("D:\\Studies\\ak101\\202\\stat\\CSR\\product\\program\\macros").exists());
    }
}
