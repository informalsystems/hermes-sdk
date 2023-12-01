use cgp_core::prelude::*;

pub struct WalletPrefix {
    pub prefix: String,
    pub is_validator: bool,
}

pub trait HasGenesisWalletPrefixes: Async {
    fn genesis_wallet_prefixes(&self) -> Vec<WalletPrefix>;
}
