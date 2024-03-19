use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::types::signer::{HasSignerType, SignerOf};

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::tx_context::HasTxContextType;

pub type Wallet<Chain> = <Chain as HasWalletType>::Wallet;

#[derive_component(WalletTypeComponent, ProvideWalletType<Chain>)]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

pub type WalletOf<ChainDriver> = <ChainDriver as HasWalletType>::Wallet;

#[derive_component(WalletSignerComponent, WalletSignerProvider<Chain>)]
pub trait HasWalletSigner: HasWalletType + HasTxContextType
where
    Self::TxContext: HasSignerType,
{
    fn wallet_signer(wallet: &Self::Wallet) -> &SignerOf<Self::TxContext>;
}
