use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::chain::HasChainType;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[derive_component(ChainDataInitializerComponent, ChainDataInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitChainData: HasRuntime + HasChainType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_data(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
