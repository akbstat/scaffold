mod generator;
mod reader;
mod render;
mod skeleton;
mod template;

pub use generator::{FileResult, Generator, Group, Param};
pub use reader::list_projects;
pub use reader::Kind;
pub use skeleton::{Builder, DocumentSkeleton, StatSkeleton, STAT};
pub use template::{Version, VersionManager, VersionManagerParam};
