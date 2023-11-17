use std::path::Path;

use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

#[derive_component(InitChainCommandRunnerComponent, InitChainCommandRunner<Bootstrap>)]
#[async_trait]
pub trait CanRunInitChainCommand: HasChainIdType + HasErrorType {
    async fn run_init_chain_command(
        &self,
        chain_id: &Self::ChainId,
        chain_home_dir: &Path,
    ) -> Result<(), Self::Error>;
}
