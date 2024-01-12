use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::types::{HasSignerType, SignerOf};

use crate::chain::traits::types::address::HasAddressType;
use crate::driver::traits::types::chain::HasChainType;

pub type Wallet<Chain> = <Chain as HasWalletType>::Wallet;

#[derive_component(WalletTypeComponent, WalletTypeProvider<Chain>)]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

#[derive_component(WalletSignerComponent, WalletSignerProvider<Chain>)]
pub trait HasWalletSigner: HasWalletType + HasChainType
where
    Self::Chain: HasSignerType,
{
    fn wallet_signer(wallet: &Self::Wallet) -> &SignerOf<Self::Chain>;
}
