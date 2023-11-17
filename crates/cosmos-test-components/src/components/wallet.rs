use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_components::transaction::traits::types::HasSignerType;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::wallet::{
    HasWalletType, WalletSignerProvider, WalletTypeProvider,
};

use crate::types::wallet::Wallet;

pub struct ProvideKeyPairWallet;

impl<Chain> WalletTypeProvider<Chain> for ProvideKeyPairWallet
where
    Chain: HasAddressType<Address = String>,
{
    type Wallet = Wallet;

    fn wallet_address(wallet: &Self::Wallet) -> &String {
        &wallet.address
    }
}

impl<Chain> WalletSignerProvider<Chain> for ProvideKeyPairWallet
where
    Chain: HasWalletType<Wallet = Wallet> + HasSignerType<Signer = Secp256k1KeyPair>,
{
    fn wallet_signer(wallet: &Wallet) -> &Secp256k1KeyPair {
        &wallet.keypair
    }
}
