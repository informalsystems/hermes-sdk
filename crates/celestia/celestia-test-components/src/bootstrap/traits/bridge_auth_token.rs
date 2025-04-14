use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use crate::bridge_driver::traits::bridge_auth_token::{BridgeAuthTokenOf, HasBridgeAuthTokenType};

#[cgp_component {
  provider: BridgeAuthTokenGenerator,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanGenerateBridgeAuthToken:
    HasRuntimeType + HasChainType + HasBridgeDriverType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
    Self::BridgeDriver: HasBridgeAuthTokenType,
{
    async fn generate_bridge_auth_token(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<BridgeAuthTokenOf<Self::BridgeDriver>, Self::Error>;
}
