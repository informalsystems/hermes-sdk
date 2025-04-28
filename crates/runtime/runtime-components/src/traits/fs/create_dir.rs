use hermes_prelude::*;

use crate::traits::HasFilePathType;

#[cgp_component {
  provider: DirCreator,
  context: Runtime,
}]
#[async_trait]
pub trait CanCreateDir: HasFilePathType + HasAsyncErrorType {
    async fn create_dir(&self, dir_path: &Self::FilePath) -> Result<(), Self::Error>;
}
