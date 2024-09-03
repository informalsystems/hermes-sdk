use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;

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
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Wallet<Self::Chain>, Self::Error>;
}
