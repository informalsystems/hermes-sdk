use alloc::boxed::Box;

use cgp_core::prelude::*;

use crate::runtime::traits::types::file_path::HasFilePathType;

#[derive_component(StringToFileWriterComponent, StringToFileWriter<Runtime>)]
#[async_trait]
pub trait CanWriteStringToFile: HasFilePathType + HasErrorType {
    async fn write_string_to_file(
        &self,
        path: &Self::FilePath,
        content: &str,
    ) -> Result<(), Self::Error>;
}
