use std::path::Path;
use std::process::Child;

use cgp_core::prelude::*;

#[async_trait]
pub trait CanBuildChainFromCosmosConfig: HasErrorType {
    async fn build_chain_from_cosmos_config(
        command_path: &Path,
        home_path: &Path,
        chain_id: &str,
        account_prefix: &str,
        chain_process: Child,
    ) -> Result<Self, Self::Error>;
}
