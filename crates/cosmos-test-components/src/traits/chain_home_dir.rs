use std::path::PathBuf;

use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

#[async_trait]
pub trait CanAllocateChainHomeDir: HasChainIdType + HasErrorType {
    async fn allocate_chain_home_dir(
        &self,
        chain_id: &Self::ChainId,
    ) -> Result<PathBuf, Self::Error>;
}
