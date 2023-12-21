use alloc::string::String;

use cgp_core::prelude::*;

#[derive_component(FilePathTypeComponent, ProvideFilePathType<Runtime>)]
pub trait HasFilePathType: Async {
    type FilePath: Async;

    fn file_path_from_string(path: &str) -> Self::FilePath;

    fn file_path_to_string(path: &Self::FilePath) -> String;

    fn join_file_path(path1: &Self::FilePath, path2: &Self::FilePath) -> Self::FilePath;
}

pub type FilePath<Runtime> = <Runtime as HasFilePathType>::FilePath;
