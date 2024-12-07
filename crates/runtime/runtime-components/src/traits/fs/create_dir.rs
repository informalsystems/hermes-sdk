use cgp::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[cgp_component {
  name: DirCreatorComponent,
  provider: DirCreator,
  context: Runtime,
}]
#[async_trait]
pub trait CanCreateDir: HasFilePathType + HasErrorType {
    async fn create_dir(&self, dir_path: &Self::FilePath) -> Result<(), Self::Error>;
}
