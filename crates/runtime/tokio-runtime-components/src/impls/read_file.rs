use std::io::Error as IoError;
use std::path::Path;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_test_components::runtime::traits::read_file::FileAsStringReader;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use tokio::fs::read_to_string;

pub struct TokioReadFileAsString;

#[async_trait]
impl<Runtime> FileAsStringReader<Runtime> for TokioReadFileAsString
where
    Runtime: HasFilePathType + CanRaiseError<IoError>,
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
