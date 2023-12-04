use cgp_core::prelude::*;

use crate::traits::types::io::file_path::HasFilePathType;

#[async_trait]
pub trait CanReadFileAsString: HasFilePathType + HasErrorType {
    async fn read_file_as_string(&self, file_path: &Self::FilePath) -> Result<String, Self::Error>;
}
