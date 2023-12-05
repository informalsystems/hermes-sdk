use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::chain::HasChainType;

use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[async_trait]
pub trait CanInitChainHomeDir: HasChainType + HasRuntime + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_home_dir(
        &self,
        chain_id: &ChainId<Self::Chain>,
    ) -> Result<FilePath<Self::Runtime>, Self::Error>;
}
