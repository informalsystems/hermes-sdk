use hermes_prelude::*;

use crate::traits::HasFilePathType;

#[cgp_component {
  provider: StringToFileWriter,
  context: Runtime,
}]
#[async_trait]
pub trait CanWriteStringToFile: HasFilePathType + HasAsyncErrorType {
    async fn write_string_to_file(
        &self,
        path: &Self::FilePath,
        content: &str,
    ) -> Result<(), Self::Error>;
}
