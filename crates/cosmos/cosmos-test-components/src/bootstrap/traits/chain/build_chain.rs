use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, Wallet};

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::chain_config::HasChainConfigType;
use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;

#[derive_component(ChainFromBootstrapParamsBuilderComponent, ChainFromBootstrapParamsBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildChainFromBootstrapParameters:
    HasRuntime + HasChainDriverType + HasGenesisConfigType + HasChainConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType + HasChildProcessType,
    Self::Chain: HasChainIdType,
    Self::ChainDriver: HasWalletType,
{
    async fn build_chain_from_bootstrap_params(
        &self,
        chain_home_dir: FilePathOf<Self::Runtime>,
        chain_id: ChainIdOf<Self::Chain>,
        genesis_config: Self::GenesisConfig,
        chain_config: Self::ChainConfig,
        wallets: Vec<Wallet<Self::ChainDriver>>,
        chain_processes: Vec<ChildProcessOf<Self::Runtime>>,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
