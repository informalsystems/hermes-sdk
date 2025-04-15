use std::io::Error as IoError;
use std::path::Path;

use cgp::prelude::*;
use hermes_runtime_components::traits::{DirCreator, DirCreatorComponent, HasFilePathType};
use tokio::fs::create_dir_all;

pub struct TokioCreateDir;

#[cgp_provider(DirCreatorComponent)]
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
