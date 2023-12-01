use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::types::file_path::HasFilePathType;

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[derive_component(ChainDataInitializerComponent, ChainDataInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitChainData: HasChainIdType + HasFilePathType + HasErrorType {
    async fn init_chain_data(
        &self,
        chain_id: &Self::ChainId,
        chain_home_dir: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}
