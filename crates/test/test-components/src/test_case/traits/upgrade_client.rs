use hermes_chain_type_components::traits::HasClientIdType;
use hermes_prelude::*;

#[cgp_component {
  provider: UpgradeClientHandler,
  context: Driver,
}]
#[async_trait]
pub trait CanHandleUpgradeClient<ChainDriverA, ChainA, ChainB>: HasAsyncErrorType
where
    ChainA: HasClientIdType<ChainB>,
{
    async fn handle_upgrade_client(&self) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: SetupUpgradeClientTestHandler,
  context: Driver,
}]
#[async_trait]
pub trait CanSetupUpgradeClientTest<ChainDriverA, ChainA, ChainB>: HasAsyncErrorType
where
    ChainA: HasClientIdType<ChainB>,
{
    async fn setup_upgrade_client_test(&self) -> Result<(), Self::Error>;
}
