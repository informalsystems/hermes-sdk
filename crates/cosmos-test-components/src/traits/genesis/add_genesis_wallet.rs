use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::types::file_path::HasFilePathType;
use crate::traits::types::wallet_config::HasWalletConfigType;

#[derive_component(GenesisWalletAdderComponent, GenesisWalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasFilePathType + HasChainIdType + HasWalletType + HasWalletConfigType + HasErrorType
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &Self::FilePath,
        chain_id: &Self::ChainId,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Self::Wallet, Self::Error>;
}
