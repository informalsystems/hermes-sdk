use std::path::Path;

use cgp_core::prelude::*;

#[derive_component(AddWalletSeedCommandRunnerComponent, AddWalletSeedCommandRunner<Bootstrap>)]
#[async_trait]
pub trait CanRunAddWalletSeedCommand: HasErrorType {
    async fn run_add_wallet_seed_command(
        &self,
        chain_home_dir: &Path,
        wallet_id: &str,
    ) -> Result<String, Self::Error>;
}
