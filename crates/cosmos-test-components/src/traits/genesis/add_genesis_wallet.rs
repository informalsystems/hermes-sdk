use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};
use crate::traits::types::wallet_config::HasWalletConfigType;

#[derive_component(GenesisWalletAdderComponent, GenesisWalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasRuntime + HasChainIdType + HasWalletType + HasWalletConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &Self::ChainId,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Self::Wallet, Self::Error>;
}
