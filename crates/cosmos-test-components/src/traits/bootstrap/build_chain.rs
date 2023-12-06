use std::process::Child;

use cgp_core::prelude::*;

use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::runtime::traits::types::child_process::HasChildProcessType;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

use crate::traits::types::chain_config::HasChainConfigType;
use crate::traits::types::genesis_config::HasGenesisConfigType;

#[async_trait]
pub trait CanBuildChainFromBootstrapConfig:
    HasRuntime + HasChainType + HasGenesisConfigType + HasChainConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType + HasChildProcessType,
    Self::Chain: HasChainIdType,
{
    async fn build_chain_from_bootstrap_config(
        command_path: &FilePath<Self::Runtime>,
        home_path: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
        chain_process: Child,
    ) -> Result<Self, Self::Error>;
}
