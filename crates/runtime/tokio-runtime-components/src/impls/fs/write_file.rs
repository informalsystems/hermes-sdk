use std::io::Error as IoError;
use std::path::Path;

use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::fs::write_file::StringToFileWriter;
use tokio::fs::write;

pub struct TokioWriteStringToFile;

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
