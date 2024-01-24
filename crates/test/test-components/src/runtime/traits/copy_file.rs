use cgp_core::prelude::*;

use crate::runtime::traits::types::file_path::HasFilePathType;

#[derive_component(FileCopierComponent, FileCopier<Runtime>)]
#[async_trait]
pub trait CanCopyFile: HasFilePathType + HasErrorType {
    async fn copy_file(
        &self,
        source_path: &Self::FilePath,
        destination_path: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}
