use std::path::PathBuf;

use cgp::prelude::*;
use hermes_runtime_components::traits::{FilePathTypeComponent, ProvideFilePathType};

pub struct ProvideStdPathType;

#[cgp_provider(FilePathTypeComponent)]
impl<Runtime> ProvideFilePathType<Runtime> for ProvideStdPathType
where
    Runtime: Async,
{
    type FilePath = PathBuf;

    fn file_path_from_string(path: &str) -> PathBuf {
        path.into()
    }

    fn file_path_to_string(path: &PathBuf) -> String {
        path.to_string_lossy().to_string()
    }

    fn join_file_path(path1: &PathBuf, path2: &PathBuf) -> PathBuf {
        path1.join(path2)
    }
}
