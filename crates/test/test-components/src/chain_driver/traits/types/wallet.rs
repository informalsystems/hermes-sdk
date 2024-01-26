use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::types::{HasSignerType, SignerOf};

use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::tx_context::HasTxContextType;

pub type Wallet<Chain> = <Chain as HasWalletType>::Wallet;

#[derive_component(WalletTypeComponent, WalletTypeProvider<Chain>)]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

#[derive_component(WalletSignerComponent, WalletSignerProvider<Chain>)]
pub trait HasWalletSigner: HasWalletType + HasTxContextType
where
    Self::TxContext: HasSignerType,
{
    fn wallet_signer(wallet: &Self::Wallet) -> &SignerOf<Self::TxContext>;
}
