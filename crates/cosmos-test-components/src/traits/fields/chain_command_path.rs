use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};

pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePath<Self::Runtime>;
}
