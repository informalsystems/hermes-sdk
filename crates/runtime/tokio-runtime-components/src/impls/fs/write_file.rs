use std::io::Error as IoError;
use std::path::Path;

use cgp::prelude::*;
use hermes_runtime_components::traits::{
    HasFilePathType, StringToFileWriter, StringToFileWriterComponent,
};
use tokio::fs::write;

pub struct TokioWriteStringToFile;

#[cgp_provider(StringToFileWriterComponent)]
impl<Runtime> StringToFileWriter<Runtime> for TokioWriteStringToFile
where
    Runtime: HasFilePathType + HasAsyncErrorType,
    Runtime::FilePath: AsRef<Path>,
    Runtime: CanRaiseAsyncError<IoError>,
{
    async fn write_string_to_file(
        _runtime: &Runtime,
        path: &Runtime::FilePath,
        content: &str,
    ) -> Result<(), Runtime::Error> {
        write(path, content).await.map_err(Runtime::raise_error)?;

        Ok(())
    }
}
