use cgp_core::prelude::*;

use crate::chain::traits::types::wallet::HasWalletType;
use crate::types::index::Index;

#[derive(Copy, Clone)]
pub struct UserWallet;

#[derive(Copy, Clone)]
pub struct RelayerWallet;

#[derive_component(WalletFieldComponent, ProvideWallet<Chain>)]
pub trait HasWalletAt<WalletKind, const I: usize>: HasWalletType {
    fn wallet_at(&self, kind: WalletKind, index: Index<I>) -> &Self::Wallet;
}
