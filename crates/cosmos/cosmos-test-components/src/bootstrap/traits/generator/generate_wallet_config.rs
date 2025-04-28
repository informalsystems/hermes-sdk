use alloc::collections::BTreeMap;

use hermes_prelude::*;

use crate::bootstrap::traits::{HasChainGenesisConfigType, HasWalletConfigType};

#[cgp_component {
  provider: WalletConfigGenerator,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanGenerateWalletConfigs:
    HasChainGenesisConfigType + HasWalletConfigType + HasAsyncErrorType
{
    async fn generate_wallet_configs(
        &self,
        genesis_config: &Self::ChainGenesisConfig,
    ) -> Result<BTreeMap<String, Self::WalletConfig>, Self::Error>;
}
