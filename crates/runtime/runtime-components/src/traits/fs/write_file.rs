use cgp::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[cgp_component {
  name: StringToFileWriterComponent,
  provider: StringToFileWriter,
  context: Runtime,
}]
#[async_trait]
pub trait CanWriteStringToFile: HasFilePathType + HasErrorType {
    async fn write_string_to_file(
        &self,
        path: &Self::FilePath,
        content: &str,
    ) -> Result<(), Self::Error>;
}
