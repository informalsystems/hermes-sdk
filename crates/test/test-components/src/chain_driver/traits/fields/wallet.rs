use alloc::collections::BTreeMap;
use alloc::string::String;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::wallet::{HasWalletType, WalletOf};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive(Copy, Clone)]
pub struct UserWallet;

#[derive(Copy, Clone)]
pub struct RelayerWallet;

#[derive(Copy, Clone)]
pub struct ValidatorWallet;

#[cgp_component {
  name: WalletGetterComponent,
  provider: WalletGetterAt,
  context: ChainDriver,
}]
pub trait HasWalletAt<WalletKind, I: Async>: HasChainType
where
    Self::Chain: HasWalletType,
{
    fn wallet_at(&self, _kind: WalletKind, _index: PhantomData<I>) -> &WalletOf<Self::Chain>;
}

#[cgp_component {
  provider: WalletsGetter,
  context: ChainDriver,
}]
pub trait HasWallets: HasChainType
where
    Self::Chain: HasWalletType,
{
    fn wallets(&self) -> &BTreeMap<String, WalletOf<Self::Chain>>;
}
