use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};

#[async_trait]
pub trait CanInitChainHomeDir: HasChainIdType + HasRuntime + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_chain_home_dir(
        &self,
        chain_id: &Self::ChainId,
    ) -> Result<FilePath<Self::Runtime>, Self::Error>;
}
