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
}

pub trait HasWalletConfigFields: HasWalletConfigType {
    /// Get the ID for identifying the wallet from the chain configuration.
    /// This can be used for locating the private key of the wallet.
    fn wallet_config_wallet_id(wallet_config: &Self::WalletConfig) -> &str;

    /// Get the balances that the wallet should have in genesis.
    /// The returned amounts should be in different denoms, or the
    /// gentx command may fail.
    fn wallet_config_genesis_balances(wallet_config: &Self::WalletConfig) -> &[Self::Amount];

    /// If the wallet is a validator, returns an amount that should be staked
    /// on genesis. The amount should be in the native staking denom,
    /// or the bootstrapping may fail.
    fn wallet_config_validator_staked_amount(
        wallet_config: &Self::WalletConfig,
    ) -> Option<&Self::Amount>;
}
