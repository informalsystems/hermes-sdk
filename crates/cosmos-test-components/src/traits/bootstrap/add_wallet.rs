use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::types::file_path::HasFilePathType;

#[derive_component(WalletAdderComponent, WalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWallet: HasWalletType + HasFilePathType + HasErrorType {
    async fn add_wallet(
        &self,
        chain_home_dir: &Self::FilePath,
        wallet_id: &str,
    ) -> Result<Self::Wallet, Self::Error>;
}
