use std::path::Path;

use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

#[derive_component(WalletAdderComponent, WalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWallet: HasWalletType + HasErrorType {
    async fn run_add_wallet_command(
        &self,
        chain_home_dir: &Path,
        wallet_id: &str,
    ) -> Result<Self::Wallet, Self::Error>;
}
