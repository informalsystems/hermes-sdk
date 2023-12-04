use std::process::Child;

use cgp_core::prelude::*;

use crate::traits::types::io::file_path::HasFilePathType;

#[async_trait]
pub trait CanBuildChainFromCosmosConfig: HasFilePathType + HasErrorType {
    async fn build_chain_from_cosmos_config(
        command_path: &Self::FilePath,
        home_path: &Self::FilePath,
        chain_id: &str,
        account_prefix: &str,
        chain_process: Child,
    ) -> Result<Self, Self::Error>;
}
