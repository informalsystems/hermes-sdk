use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::chain::HasChainType;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::types::wallet_config::HasWalletConfigType;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisWalletAdderComponent, GenesisWalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasRuntime + HasChainType + HasWalletType + HasWalletConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Self::Wallet, Self::Error>;
}
