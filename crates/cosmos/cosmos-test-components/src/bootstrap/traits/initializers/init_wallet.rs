use cgp::prelude::*;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain::traits::{HasWalletType, Wallet};
use hermes_core::test_components::chain_driver::traits::HasChainType;

#[cgp_component {
  provider: WalletInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitWallet: HasRuntime + HasChainType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType,
{
    async fn initialize_wallet(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        wallet_id: &str,
    ) -> Result<Wallet<Self::Chain>, Self::Error>;
}
