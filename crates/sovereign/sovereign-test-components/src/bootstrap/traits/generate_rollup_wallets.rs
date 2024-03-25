use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};

#[derive_component(RollupWalletGeneratorComponent, RollupWalletGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateRollupWallets: HasRollupType + HasErrorType
where
    Self::Rollup: HasWalletType,
{
    async fn generate_rollup_wallets(
        &self,
    ) -> Result<BTreeMap<String, WalletOf<Self::Rollup>>, Self::Error>;
}
