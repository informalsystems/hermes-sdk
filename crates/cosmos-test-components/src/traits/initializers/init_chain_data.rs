use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::types::io::file_path::HasFilePathType;

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[derive_component(ChainDataInitializerComponent, ChainDataInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitChainData: HasChainIdType + HasFilePathType + HasErrorType {
    async fn init_chain_data(
        &self,
        chain_home_dir: &Self::FilePath,
        chain_id: &Self::ChainId,
    ) -> Result<(), Self::Error>;
}
