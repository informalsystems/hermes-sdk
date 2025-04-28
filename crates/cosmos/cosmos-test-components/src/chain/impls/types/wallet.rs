use cgp::prelude::*;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::relayer_components::transaction::traits::HasSignerType;
use hermes_core::test_components::chain::traits::{
    HasWalletType, ProvideWalletType, WalletSignerComponent, WalletSignerProvider,
    WalletTypeComponent,
};
use hermes_cosmos_chain_components::types::Secp256k1KeyPair;

use crate::chain::types::CosmosTestWallet;

pub struct ProvideCosmosTestWallet;

#[cgp_provider(WalletTypeComponent)]
impl<Chain> ProvideWalletType<Chain> for ProvideCosmosTestWallet
where
    Chain: HasAddressType<Address = String>,
{
    type Wallet = CosmosTestWallet;

    fn wallet_address(wallet: &Self::Wallet) -> &String {
        &wallet.address
    }
}

#[cgp_provider(WalletSignerComponent)]
impl<Chain> WalletSignerProvider<Chain> for ProvideCosmosTestWallet
where
    Chain: HasWalletType<Wallet = CosmosTestWallet> + HasSignerType<Signer = Secp256k1KeyPair>,
{
    fn wallet_signer(wallet: &CosmosTestWallet) -> &Secp256k1KeyPair {
        &wallet.keypair
    }
}
