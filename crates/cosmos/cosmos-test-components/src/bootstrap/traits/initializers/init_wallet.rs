use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(WalletInitializerComponent, WalletInitializer<Bootstrap>)]
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
