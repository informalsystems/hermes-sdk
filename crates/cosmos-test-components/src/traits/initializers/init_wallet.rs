use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};

#[derive_component(WalletInitializerComponent, WalletInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitWallet: HasWalletType + HasRuntime + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn initialize_wallet(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        wallet_id: &str,
    ) -> Result<Self::Wallet, Self::Error>;
}
