use hermes_prelude::*;

use crate::chain_driver::traits::HasSetupUpgradeClientTestResultType;

#[cgp_component {
  provider: UpgradeClientHandler,
  context: Driver,
}]
#[async_trait]
pub trait CanHandleUpgradeClient<ChainDriverA, ChainA, ChainB>: HasAsyncErrorType
where
    ChainDriverA: HasSetupUpgradeClientTestResultType,
{
    async fn handle_upgrade_client(
        &self,
        setup_result: &ChainDriverA::SetupUpgradeClientTestResult,
    ) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: SetupUpgradeClientTestHandler,
  context: Driver,
}]
#[async_trait]
pub trait CanSetupUpgradeClientTest<ChainDriverA, ChainA, ChainB>: HasAsyncErrorType
where
    ChainDriverA: HasSetupUpgradeClientTestResultType,
{
    async fn setup_upgrade_client_test(
        &self,
    ) -> Result<ChainDriverA::SetupUpgradeClientTestResult, Self::Error>;
}
