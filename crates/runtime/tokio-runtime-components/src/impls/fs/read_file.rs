use std::io::Error as IoError;
use std::path::Path;

use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::fs::read_file::FileAsStringReader;
use tokio::fs::read_to_string;

pub struct TokioReadFileAsString;

impl<Runtime> FileAsStringReader<Runtime> for TokioReadFileAsString
where
    Runtime: HasFilePathType + CanRaiseAsyncError<IoError>,
    Runtime::FilePath: AsRef<Path>,
{
    async fn read_file_as_string(
        _runtime: &Runtime,
        file_path: &Runtime::FilePath,
    ) -> Result<String, Runtime::Error> {
        let content = read_to_string(file_path)
            .await
            .map_err(Runtime::raise_error)?;

        Ok(content)
    }
}
