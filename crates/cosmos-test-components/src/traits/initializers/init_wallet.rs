use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::types::file_path::HasFilePathType;

#[derive_component(WalletInitializerComponent, WalletInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitWallet: HasWalletType + HasFilePathType + HasErrorType {
    async fn initialize_wallet(
        &self,
        chain_home_dir: &Self::FilePath,
        wallet_id: &str,
    ) -> Result<Self::Wallet, Self::Error>;
}
