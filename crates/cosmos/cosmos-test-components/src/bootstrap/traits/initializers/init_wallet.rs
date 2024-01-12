use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(WalletInitializerComponent, WalletInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitWallet: HasRuntime + HasChainDriverType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::ChainDriver: HasWalletType,
{
    async fn initialize_wallet(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        wallet_id: &str,
    ) -> Result<Wallet<Self::ChainDriver>, Self::Error>;
}
