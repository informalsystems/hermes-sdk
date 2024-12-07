use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
  name: WalletInitializerComponent,
  provider: WalletInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitWallet: HasRuntime + HasChainType + HasErrorType
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
