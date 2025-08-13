use hermes_chain_type_components::traits::HasClientIdType;
use hermes_prelude::*;

use crate::chain_driver::traits::{HasChainType, HasSetupUpgradeClientTestResultType};

#[cgp_component {
  provider: UpgradeClientHandler,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanHandleUpgradeClient<ChainDriverB>:
    HasSetupUpgradeClientTestResultType + HasChainType + HasAsyncErrorType
where
    ChainDriverB: HasChainType<Chain: HasClientIdType<<Self as HasChainType>::Chain>> + Async,
{
    async fn handle_upgrade_client(
        &self,
        setup_result: &Self::SetupUpgradeClientTestResult,
        chain_driver_b: &ChainDriverB,
        client_id_b: &<<ChainDriverB as HasChainType>::Chain as HasClientIdType<
            <Self as HasChainType>::Chain,
        >>::ClientId,
    ) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: SetupUpgradeClientTestHandler,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanSetupUpgradeClientTest<ChainDriverB>:
    HasSetupUpgradeClientTestResultType + HasChainType + HasAsyncErrorType
where
    ChainDriverB: HasChainType<Chain: HasClientIdType<<Self as HasChainType>::Chain>> + Async,
{
    async fn setup_upgrade_client_test(
        &self,
        chain_driver_b: &ChainDriverB,
        client_id_b: &<<ChainDriverB as HasChainType>::Chain as HasClientIdType<
            <Self as HasChainType>::Chain,
        >>::ClientId,
    ) -> Result<Self::SetupUpgradeClientTestResult, Self::Error>;
}
