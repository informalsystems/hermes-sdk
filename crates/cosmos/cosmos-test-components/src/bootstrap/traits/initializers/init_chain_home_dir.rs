use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(ChainHomeDirInitializerComponent, ChainHomeDirInitializer<Bootstrap>)]
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
