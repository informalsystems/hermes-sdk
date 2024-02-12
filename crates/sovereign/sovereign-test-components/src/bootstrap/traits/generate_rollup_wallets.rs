use alloc::collections::BTreeMap;
use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, WalletOf};

use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;

#[derive_component(RollupWalletGeneratorComponent, RollupWalletGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateRollupWallets: HasRollupDriverType + HasErrorType
where
    Self::RollupDriver: HasWalletType,
{
    async fn generate_rollup_wallets(
        &self,
    ) -> Result<BTreeMap<String, WalletOf<Self::RollupDriver>>, Self::Error>;
}
