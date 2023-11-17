use cgp_core::prelude::*;
use ibc_relayer_components::transaction::traits::types::HasSignerType;

use crate::traits::chain::types::address::HasAddressType;

#[derive_component(WalletTypeComponent, WalletTypeProvider<Chain>)]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

#[derive_component(WalletSignerComponent, WalletSignerProvider<Chain>)]
pub trait HasWalletSigner: HasWalletType + HasSignerType {
    fn wallet_signer(wallet: &Self::Wallet) -> &Self::Signer;
}
