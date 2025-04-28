use alloc::collections::BTreeMap;
use alloc::string::String;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::{HasWalletType, WalletOf};
use crate::chain_driver::traits::HasChainType;

#[derive(Copy, Clone)]
pub struct UserWallet<const I: usize = 0>;

#[derive(Copy, Clone)]
pub struct RelayerWallet;

#[derive(Copy, Clone)]
pub struct ValidatorWallet;

#[cgp_getter {
    name: WalletGetterComponent<Tag>,
    provider: WalletGetter,
}]
pub trait HasWallet<Tag>: HasChainType<Chain: HasWalletType> {
    fn wallet(&self, _tag: PhantomData<Tag>) -> &WalletOf<Self::Chain>;
}

#[cgp_getter {
    provider: WalletsGetter,
}]
pub trait HasWallets: HasChainType<Chain: HasWalletType> {
    fn wallets(&self) -> &BTreeMap<String, WalletOf<Self::Chain>>;
}
