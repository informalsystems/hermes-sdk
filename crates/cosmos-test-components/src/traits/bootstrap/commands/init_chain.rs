use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::file_path::HasFilePathType;

#[derive_component(InitChainCommandRunnerComponent, InitChainCommandRunner<Bootstrap>)]
#[async_trait]
pub trait CanRunInitChainCommand: HasChainIdType + HasFilePathType + HasErrorType {
    async fn run_init_chain_command(
        &self,
        chain_id: &Self::ChainId,
        chain_home_dir: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}
