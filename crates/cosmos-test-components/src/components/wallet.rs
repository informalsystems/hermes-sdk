use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::wallet::WalletTypeProvider;

pub struct ProvideKeyPairWallet;

impl<Chain> WalletTypeProvider<Chain> for ProvideKeyPairWallet
where
    Chain: HasAddressType<Address = String>,
{
    type Wallet = (String, Secp256k1KeyPair);

    fn wallet_address(wallet: &Self::Wallet) -> &String {
        &wallet.0
    }
}
