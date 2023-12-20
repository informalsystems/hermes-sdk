use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};

use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(WalletInitializerComponent, WalletInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitWallet: HasRuntime + HasChainType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType,
{
    async fn initialize_wallet(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        wallet_id: &str,
    ) -> Result<Wallet<Self::Chain>, Self::Error>;
}
