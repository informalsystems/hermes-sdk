use crate::chain::traits::types::wallet::HasWalletType;

pub struct UserWallet;

pub struct RelayerWallet;

pub trait HasWallet<WalletKind, const I: usize>: HasWalletType {
    fn user_wallet(&self) -> &Self::Wallet;
}

pub trait HasOneUserWallet: HasWallet<UserWallet, 0> {
    fn first_user_wallet(&self) -> &Self::Wallet;
}

impl<Chain> HasOneUserWallet for Chain
where
    Chain: HasWallet<UserWallet, 0>,
{
    fn first_user_wallet(&self) -> &Self::Wallet {
        self.user_wallet()
    }
}

pub trait HasTwoUserWallets: HasWallet<UserWallet, 1> + HasOneUserWallet {
    fn second_user_wallet(&self) -> &Self::Wallet;
}

impl<Chain> HasTwoUserWallets for Chain
where
    Chain: HasWallet<UserWallet, 0> + HasWallet<UserWallet, 1>,
{
    fn second_user_wallet(&self) -> &Self::Wallet {
        self.nth_user_wallet::<1>()
    }
}

/// Helper auto trait method for accessing N-th user wallet
pub trait NthUserWallet: HasWalletType {
    fn nth_user_wallet<const I: usize>(&self) -> &Self::Wallet
    where
        Self: HasWallet<UserWallet, I>;
}

impl<Chain> NthUserWallet for Chain
where
    Chain: HasWalletType,
{
    fn nth_user_wallet<const I: usize>(&self) -> &Self::Wallet
    where
        Self: HasWallet<UserWallet, I>,
    {
        self.user_wallet()
    }
}
