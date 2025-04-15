use cgp::prelude::*;

use crate::traits::HasFilePathType;

#[cgp_component {
  provider: FileCopier,
  context: Runtime,
}]
#[async_trait]
pub trait CanCopyFile: HasFilePathType + HasAsyncErrorType {
    async fn copy_file(
        &self,
        source_path: &Self::FilePath,
        destination_path: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}
