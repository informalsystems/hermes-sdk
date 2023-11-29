use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::file_path::HasFilePathType;

#[async_trait]
pub trait CanAllocateChainHomeDir: HasChainIdType + HasFilePathType + HasErrorType {
    async fn allocate_chain_home_dir(
        &self,
        chain_id: &Self::ChainId,
    ) -> Result<Self::FilePath, Self::Error>;
}
