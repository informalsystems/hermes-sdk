use std::io::Error as IoError;
use std::path::Path;

use cgp_core::error::CanRaiseError;
use hermes_runtime_components::traits::fs::copy_file::FileCopier;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use tokio::fs::copy;

pub struct TokioCopyFile;

impl<Runtime> FileCopier<Runtime> for TokioCopyFile
where
    Runtime: HasFilePathType + CanRaiseError<IoError>,
    Runtime::FilePath: AsRef<Path>,
{
    async fn copy_file(
        _runtime: &Runtime,
        source_path: &Runtime::FilePath,
        destination_path: &Runtime::FilePath,
    ) -> Result<(), Runtime::Error> {
        copy(source_path, destination_path)
            .await
            .map_err(Runtime::raise_error)?;

        Ok(())
    }
}
