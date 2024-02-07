use alloc::collections::BTreeMap;
use cgp_core::prelude::*;

use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;

#[derive_component(WalletConfigGeneratorComponent, WalletConfigGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateWalletConfigs:
    HasChainGenesisConfigType + HasWalletConfigType + HasErrorType
{
    async fn generate_wallet_configs(
        &self,
        genesis_config: &Self::ChainGenesisConfig,
    ) -> Result<BTreeMap<String, Self::WalletConfig>, Self::Error>;
}
