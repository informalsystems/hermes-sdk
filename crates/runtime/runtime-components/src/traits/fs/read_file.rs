use alloc::string::String;

use cgp_core::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[derive_component(FileAsStringReaderComponent, FileAsStringReader<Runtime>)]
#[async_trait]
pub trait CanReadFileAsString: HasFilePathType + HasErrorType {
    async fn read_file_as_string(&self, file_path: &Self::FilePath) -> Result<String, Self::Error>;
}
