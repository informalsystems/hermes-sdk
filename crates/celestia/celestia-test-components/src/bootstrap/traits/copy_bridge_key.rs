use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[derive_component(BridgeKeyCopierComponent, BridgeKeyCopier<Bootstrap>)]
#[async_trait]
pub trait CanCopyBridgeKey: HasRuntime + HasChainType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn copy_bridge_key(
        &self,
        wallet_id: &str,
        chain_id: &ChainId<Self::Chain>,
        chain_home_dir: &FilePath<Self::Runtime>,
        bridge_home_dir: &FilePath<Self::Runtime>,
    ) -> Result<(), Self::Error>;
}
