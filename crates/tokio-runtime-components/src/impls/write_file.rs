use std::io::Error as IoError;
use std::path::Path;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;
use ibc_test_components::runtime::traits::write_file::StringToFileWriter;
use tokio::fs::write;

pub struct TokioWriteStringToFile;

#[async_trait]
impl<Runtime> StringToFileWriter<Runtime> for TokioWriteStringToFile
where
    Runtime: HasFilePathType + HasErrorType,
    Runtime::FilePath: AsRef<Path>,
    Runtime: CanRaiseError<IoError>,
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
