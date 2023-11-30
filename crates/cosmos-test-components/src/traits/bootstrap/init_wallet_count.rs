use cgp_core::prelude::*;

pub trait HasInitWalletCount: Async {
    const INIT_WALLET_COUNT: usize;
}

pub trait HasInitWalletIds: HasInitWalletCount {
    fn init_wallet_ids(&self) -> [String; Self::INIT_WALLET_COUNT];
}