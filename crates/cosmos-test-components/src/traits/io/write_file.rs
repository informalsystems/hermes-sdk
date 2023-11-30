use cgp_core::prelude::*;

use crate::traits::types::file_path::HasFilePathType;

#[async_trait]
pub trait CanWriteStringToFile: HasFilePathType + HasErrorType {
    async fn write_string_to_file(
        &self,
        path: &Self::FilePath,
        content: &str,
    ) -> Result<(), Self::Error>;
}
