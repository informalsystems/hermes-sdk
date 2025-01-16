use std::io::Error as IoError;
use std::path::Path;

use cgp::core::error::CanRaiseAsyncError;
use hermes_runtime_components::traits::fs::create_dir::DirCreator;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use tokio::fs::create_dir_all;

pub struct TokioCreateDir;

impl<Runtime> DirCreator<Runtime> for TokioCreateDir
where
    Runtime: HasFilePathType + CanRaiseAsyncError<IoError>,
    Runtime::FilePath: AsRef<Path>,
{
    async fn create_dir(
        _runtime: &Runtime,
        dir_path: &Runtime::FilePath,
    ) -> Result<(), Runtime::Error> {
        create_dir_all(dir_path)
            .await
            .map_err(Runtime::raise_error)?;

        Ok(())
    }
}
