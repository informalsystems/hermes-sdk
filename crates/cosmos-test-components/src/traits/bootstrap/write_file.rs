use std::path::Path;

use cgp_core::prelude::*;

#[async_trait]
pub trait CanWriteFile: HasErrorType {
    async fn write_file(&self, path: &Path, content: &str) -> Result<(), Self::Error>;
}
