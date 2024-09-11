mod generator;
mod reader;
mod render;
mod skeleton;
mod template;

pub use generator::{Assignment, FileResult, Generator, Group, Param};
pub use reader::list_projects;
pub use reader::{new_reader, ConfigItem, Kind};
pub use skeleton::{Builder, DocumentSkeleton, StatSkeleton, STAT};
pub use template::{Version, VersionManager, VersionManagerParam};
