use cgp_core::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[derive_component(DirCreatorComponent, DirCreator<Runtime>)]
#[async_trait]
pub trait CanCreateDir: HasFilePathType + HasErrorType {
    async fn create_dir(&self, dir_path: &Self::FilePath) -> Result<(), Self::Error>;
}
