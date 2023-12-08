use cgp_core::prelude::*;

use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;

#[async_trait]
pub trait CanGenerateWalletConfigs: HasWalletConfigType + HasErrorType {
    async fn generate_wallet_configs(&self) -> Result<Vec<Self::WalletConfig>, Self::Error>;
}
