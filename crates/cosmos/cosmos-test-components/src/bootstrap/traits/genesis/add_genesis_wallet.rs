use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain::traits::{HasWalletType, Wallet};
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::HasWalletConfigType;

#[cgp_component {
  provider: GenesisWalletAdder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasRuntime + HasChainType + HasWalletConfigType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType + HasWalletType,
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Wallet<Self::Chain>, Self::Error>;
}
