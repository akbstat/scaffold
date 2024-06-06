mod generator;
mod reader;
mod render;
mod skeleton;

pub use generator::{FileResult, Generator, Group, Param};
pub use reader::list_projects;
pub use reader::Kind;
pub use skeleton::{Builder, DocumentSkeleton, StatSkeleton, STAT};
