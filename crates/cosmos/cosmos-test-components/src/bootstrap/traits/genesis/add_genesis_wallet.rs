use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;

#[derive_component(GenesisWalletAdderComponent, GenesisWalletAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddWalletToGenesis:
    HasRuntime + HasChainDriverType + HasWalletConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
    Self::ChainDriver: HasWalletType,
{
    async fn add_wallet_to_genesis(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
        wallet_config: &Self::WalletConfig,
    ) -> Result<Wallet<Self::ChainDriver>, Self::Error>;
}
