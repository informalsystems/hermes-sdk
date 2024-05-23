use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupGenesisGeneratorComponent, RollupGenesisGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateRollupGenesis:
    HasChainType + HasRollupType + HasRollupGenesisConfigType + HasErrorType
where
    Self::Chain: HasAddressType,
    Self::Rollup: HasWalletType,
{
    async fn generate_rollup_genesis(
        &self,
        sequencer_da_address: &AddressOf<Self::Chain>,
        rollup_wallets: &BTreeMap<String, WalletOf<Self::Rollup>>,
    ) -> Result<Self::RollupGenesisConfig, Self::Error>;
}
