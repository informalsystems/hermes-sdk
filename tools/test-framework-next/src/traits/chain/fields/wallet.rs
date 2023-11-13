use crate::traits::chain::types::wallet::HasWalletType;

pub trait HasUserWallet<const I: usize>: HasWalletType {
    fn get_user_wallet(&self) -> &Self::Wallet;
}

pub trait HasRelayerWallet: HasWalletType {
    fn get_relayer_wallet(&self) -> &Self::Wallet;
}
