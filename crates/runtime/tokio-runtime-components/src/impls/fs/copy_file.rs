use std::io::Error as IoError;
use std::path::Path;

use cgp::prelude::*;
use hermes_runtime_components::traits::{FileCopier, FileCopierComponent, HasFilePathType};
use tokio::fs::copy;

pub struct TokioCopyFile;

#[cgp_provider(FileCopierComponent)]
impl<Runtime> FileCopier<Runtime> for TokioCopyFile
where
    Runtime: HasFilePathType + CanRaiseAsyncError<IoError>,
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
