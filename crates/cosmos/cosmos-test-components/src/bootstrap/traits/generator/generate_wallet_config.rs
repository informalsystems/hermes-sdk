use alloc::collections::BTreeMap;

use cgp::prelude::*;

use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;

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
