use cgp_core::prelude::*;

pub trait HasFilePathType: Async {
    type FilePath: Async;

    fn file_path_from_string(path: &str) -> Self::FilePath;

    fn file_path_to_string(path: &Self::FilePath) -> String;

    fn join_file_path(path1: &Self::FilePath, path2: &Self::FilePath) -> Self::FilePath;
}
