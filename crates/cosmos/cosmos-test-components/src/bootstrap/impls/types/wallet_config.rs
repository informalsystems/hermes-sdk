use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::wallet_config::{
    HasWalletConfigType, ProvideWalletConfigType, WalletConfigFieldsGetter,
};
use crate::bootstrap::types::wallet_config::CosmosWalletConfig;
use crate::chain_driver::types::amount::Amount;

pub struct ProvideCosmosWalletConfigType;

impl<Bootstrap> ProvideWalletConfigType<Bootstrap> for ProvideCosmosWalletConfigType
where
    Bootstrap: Async,
{
    type WalletConfig = CosmosWalletConfig;
}

impl<Bootstrap> WalletConfigFieldsGetter<Bootstrap> for ProvideCosmosWalletConfigType
where
    Bootstrap: HasWalletConfigType<WalletConfig = CosmosWalletConfig> + HasChainDriverType,
    Bootstrap::ChainDriver: HasAmountType<Amount = Amount>,
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
