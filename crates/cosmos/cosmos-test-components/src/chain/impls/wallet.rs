use hermes_relayer_components::transaction::traits::types::HasSignerType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::{
    HasWalletType, WalletSignerProvider, WalletTypeProvider,
};
use hermes_test_components::driver::traits::types::chain::HasChainType;
use ibc_relayer::keyring::Secp256k1KeyPair;

use crate::chain::types::wallet::CosmosTestWallet;

pub struct ProvideCosmosTestWallet;

impl<Chain> WalletTypeProvider<Chain> for ProvideCosmosTestWallet
where
    Chain: HasAddressType<Address = String>,
{
    type Wallet = CosmosTestWallet;

    fn wallet_address(wallet: &Self::Wallet) -> &String {
        &wallet.address
    }
}

impl<ChainDriver, Chain> WalletSignerProvider<ChainDriver> for ProvideCosmosTestWallet
where
    ChainDriver: HasChainType<Chain = Chain> + HasWalletType<Wallet = CosmosTestWallet>,
    Chain: HasSignerType<Signer = Secp256k1KeyPair>,
{
    fn wallet_signer(wallet: &CosmosTestWallet) -> &Secp256k1KeyPair {
        &wallet.keypair
    }
}
