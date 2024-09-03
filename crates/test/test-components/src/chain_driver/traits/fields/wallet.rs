use alloc::collections::BTreeMap;
use alloc::string::String;

use cgp::prelude::*;
use hermes_relayer_components::multi::types::index::Index;

use crate::chain::traits::types::wallet::{HasWalletType, WalletOf};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive(Copy, Clone)]
pub struct UserWallet;

#[derive(Copy, Clone)]
pub struct RelayerWallet;

#[derive(Copy, Clone)]
pub struct ValidatorWallet;

#[derive_component(WalletGetterComponent, WalletGetterAt<ChainDriver>)]
pub trait HasWalletAt<WalletKind, const I: usize>: HasChainType
where
    Self::Chain: HasWalletType,
{
    fn wallet_at(&self, kind: WalletKind, index: Index<I>) -> &WalletOf<Self::Chain>;
}

#[derive_component(WalletsGetterComponent, WalletsGetter<ChainDriver>)]
pub trait HasWallets: HasChainType
where
    Self::Chain: HasWalletType,
{
    fn wallets(&self) -> &BTreeMap<String, WalletOf<Self::Chain>>;
}
