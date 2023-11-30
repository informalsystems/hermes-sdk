use cgp_core::prelude::*;

use crate::traits::types::file_path::HasFilePathType;

#[derive_component(AddWalletSeedCommandRunnerComponent, AddWalletSeedCommandRunner<Bootstrap>)]
#[async_trait]
pub trait CanRunAddWalletSeedCommand: HasFilePathType + HasErrorType {
    async fn run_add_wallet_seed_command(
        &self,
        chain_home_dir: &Self::FilePath,
        wallet_id: &str,
    ) -> Result<String, Self::Error>;
}
