use cgp::prelude::*;
use hermes_chain_type_components::traits::HasAmountType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::wallet_config::{
    HasWalletConfigType, ProvideWalletConfigType, WalletConfigFieldsComponent,
    WalletConfigFieldsGetter, WalletConfigTypeComponent,
};
use crate::bootstrap::types::wallet_config::CosmosWalletConfig;
use crate::chain::types::amount::Amount;

pub struct ProvideCosmosWalletConfigType;

#[cgp_provider(WalletConfigTypeComponent)]
impl<Bootstrap> ProvideWalletConfigType<Bootstrap> for ProvideCosmosWalletConfigType
where
    Bootstrap: Async,
{
    type WalletConfig = CosmosWalletConfig;
}

#[cgp_provider(WalletConfigFieldsComponent)]
impl<Bootstrap> WalletConfigFieldsGetter<Bootstrap> for ProvideCosmosWalletConfigType
where
    Bootstrap: HasWalletConfigType<WalletConfig = CosmosWalletConfig> + HasChainType,
    Bootstrap::Chain: HasAmountType<Amount = Amount>,
{
    fn wallet_config_wallet_id(wallet_config: &CosmosWalletConfig) -> &str {
        &wallet_config.wallet_id
    }

    fn wallet_config_genesis_balances(wallet_config: &CosmosWalletConfig) -> &[Amount] {
        &wallet_config.genesis_balances
    }

    fn wallet_config_validator_staked_amount(
        wallet_config: &CosmosWalletConfig,
    ) -> Option<&Amount> {
        wallet_config.validator_staked_amount.as_ref()
    }
}
