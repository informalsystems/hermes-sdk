use cgp_core::prelude::*;

use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use ibc_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

use crate::bootstrap::traits::types::chain_config::HasChainConfigType;
use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;

#[async_trait]
pub trait CanBuildChainFromBootstrapConfig:
    HasRuntime + HasChainType + HasGenesisConfigType + HasChainConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType + HasChildProcessType,
    Self::Chain: HasChainIdType + HasWalletType,
{
    async fn build_chain_from_bootstrap_config(
        &self,
        chain_home_dir: FilePath<Self::Runtime>,
        chain_id: ChainId<Self::Chain>,
        genesis_config: Self::GenesisConfig,
        chain_config: Self::ChainConfig,
        wallets: Vec<Wallet<Self::Chain>>,
        chain_process: ChildProcess<Self::Runtime>,
    ) -> Result<Self::Chain, Self::Error>;
}
