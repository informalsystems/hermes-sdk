use cgp_core::prelude::*;

pub trait HasInitWalletCount: Async {
    const INIT_WALLET_COUNT: usize;
}