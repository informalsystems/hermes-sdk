use alloc::string::String;

use hermes_prelude::*;

use crate::traits::HasFilePathType;

#[cgp_component {
  provider: FileAsStringReader,
  context: Runtime,
}]
#[async_trait]
pub trait CanReadFileAsString: HasFilePathType + HasAsyncErrorType {
    async fn read_file_as_string(&self, file_path: &Self::FilePath) -> Result<String, Self::Error>;
}
