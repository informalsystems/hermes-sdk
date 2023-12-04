use crate::traits::types::io::file_path::HasFilePathType;

pub trait HasChainCommandPath: HasFilePathType {
    fn chain_command_path(&self) -> &Self::FilePath;
}
