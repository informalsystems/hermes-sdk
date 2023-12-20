use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};

use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisWalletAdderComponent, GenesisWalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasRuntime + HasChainType + HasWalletConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType + HasWalletType,
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Wallet<Self::Chain>, Self::Error>;
}
