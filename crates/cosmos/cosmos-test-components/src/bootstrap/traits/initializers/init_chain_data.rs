use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

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
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
