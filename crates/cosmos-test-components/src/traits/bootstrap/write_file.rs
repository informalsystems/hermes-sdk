use cgp_core::prelude::*;

use crate::traits::file_path::HasFilePathType;

#[async_trait]
pub trait CanWriteFile: HasFilePathType + HasErrorType {
    async fn write_file(&self, path: &Self::FilePath, content: &str) -> Result<(), Self::Error>;
}
