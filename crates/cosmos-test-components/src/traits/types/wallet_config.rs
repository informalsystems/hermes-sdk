use cgp_core::prelude::*;

use ibc_test_components::traits::chain::types::amount::HasAmountType;

/**
   A wallet config is a template for generating fresh wallets for a chain.

   The wallet config contains information including the ID of the wallet, the
   initial balances the wallet should have in genesis, and whether the wallet
   belongs to a validator and should have an initial staked amount.
*/
pub trait HasWalletConfigType: HasAmountType {
    type WalletConfig: Async;

    fn wallet_config_wallet_id(wallet_config: &Self::WalletConfig) -> &str;

    fn wallet_config_genesis_balance(wallet_config: &Self::WalletConfig) -> &[Self::Amount];

    fn wallet_config_validator_staked_amount(
        wallet_config: &Self::WalletConfig,
    ) -> Option<&Self::Amount>;
}
