use cgp_core::prelude::*;

use crate::chain_driver::traits::types::wallet::HasWalletType;
use crate::types::index::Index;

#[derive(Copy, Clone)]
pub struct UserWallet;

#[derive(Copy, Clone)]
pub struct RelayerWallet;

#[derive_component(WalletGetterComponent, WalletGetterAt<ChainDriver>)]
pub trait HasWalletAt<WalletKind, const I: usize>: HasWalletType {
    fn wallet_at(&self, kind: WalletKind, index: Index<I>) -> &Self::Wallet;
}

#[derive_component(WalletsGetterComponent, WalletsGetter<ChainDriver>)]
pub trait HasWallets: HasWalletType {
    fn wallets(&self) -> &[Self::Wallet];
}
