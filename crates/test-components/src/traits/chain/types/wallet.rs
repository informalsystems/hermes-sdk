use cgp_core::Async;

use crate::traits::chain::types::address::HasAddressType;

pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}
