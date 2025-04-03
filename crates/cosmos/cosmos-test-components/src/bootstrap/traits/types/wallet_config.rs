use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::{AmountOf, HasAmountType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

/**
   A wallet config is a template for generating fresh wallets for a chain.

   The wallet config contains information including the ID of the wallet, the
   initial balances the wallet should have in genesis, and whether the wallet
   belongs to a validator and should have an initial staked amount.
*/
#[cgp_component {
  name: WalletConfigTypeComponent,
  provider: ProvideWalletConfigType,
  context: Bootstrap,
}]
pub trait HasWalletConfigType: Async {
    type WalletConfig: Async;
}

#[cgp_component {
  name: WalletConfigFieldsComponent,
  provider: WalletConfigFieldsGetter,
  context: Bootstrap,
}]
pub trait HasWalletConfigFields: HasWalletConfigType + HasChainType
where
    Self::Chain: HasAmountType,
{
    /// Get the ID for identifying the wallet from the chain configuration.
    /// This can be used for locating the private key of the wallet.
    fn wallet_config_wallet_id(wallet_config: &Self::WalletConfig) -> &str;

    /// Get the balances that the wallet should have in genesis.
    /// The returned amounts should be in different denoms, or the
    /// gentx command may fail.
    fn wallet_config_genesis_balances(
        wallet_config: &Self::WalletConfig,
    ) -> &[AmountOf<Self::Chain>];

    /// If the wallet is a validator, returns an amount that should be staked
    /// on genesis. The amount should be in the native staking denom,
    /// or the bootstrapping may fail.
    fn wallet_config_validator_staked_amount(
        wallet_config: &Self::WalletConfig,
    ) -> Option<&AmountOf<Self::Chain>>;
}
