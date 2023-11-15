use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;

#[derive_component(WalletTypeComponent, WalletTypeProvider<Chain>)]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}
