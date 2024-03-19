use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::tx_context::HasTxContextType;
use hermes_test_components::chain::traits::types::wallet::{
    HasWalletType, ProvideWalletType, WalletSignerProvider,
};
use ibc_relayer::keyring::Secp256k1KeyPair;

use crate::chain::types::wallet::CosmosTestWallet;

pub struct ProvideCosmosTestWallet;

impl<Chain> ProvideWalletType<Chain> for ProvideCosmosTestWallet
where
    Chain: HasAddressType<Address = String>,
{
    type Wallet = CosmosTestWallet;

    fn wallet_address(wallet: &Self::Wallet) -> &String {
        &wallet.address
    }
}

impl<ChainDriver, TxContext> WalletSignerProvider<ChainDriver> for ProvideCosmosTestWallet
where
    ChainDriver: HasTxContextType<TxContext = TxContext> + HasWalletType<Wallet = CosmosTestWallet>,
    TxContext: HasSignerType<Signer = Secp256k1KeyPair>,
{
    fn wallet_signer(wallet: &CosmosTestWallet) -> &Secp256k1KeyPair {
        &wallet.keypair
    }
}
