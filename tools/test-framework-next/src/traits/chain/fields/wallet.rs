use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::wallet::HasWalletType;

pub trait HasWalletFields: HasWalletType + HasAddressType {
    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

pub trait HasUserWallet<const I: usize>: HasWalletType {
    fn user_wallet(&self) -> &Self::Wallet;
}

pub trait HasRelayerWallet: HasWalletType {
    fn relayer_wallet(&self) -> &Self::Wallet;
}
