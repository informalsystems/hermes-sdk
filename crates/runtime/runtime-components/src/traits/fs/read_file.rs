use alloc::string::String;

use cgp::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[cgp_component {
  name: FileAsStringReaderComponent,
  provider: FileAsStringReader,
  context: Runtime,
}]
#[async_trait]
pub trait CanReadFileAsString: HasFilePathType + HasErrorType {
    async fn read_file_as_string(&self, file_path: &Self::FilePath) -> Result<String, Self::Error>;
}
