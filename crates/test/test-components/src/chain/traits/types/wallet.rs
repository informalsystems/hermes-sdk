use hermes_chain_type_components::traits::HasAddressType;
use hermes_prelude::*;
use hermes_relayer_components::transaction::traits::HasSignerType;

pub type Wallet<Chain> = <Chain as HasWalletType>::Wallet;

#[cgp_component {
  name: WalletTypeComponent,
  provider: ProvideWalletType,
  context: Chain,
}]
pub trait HasWalletType: HasAddressType {
    type Wallet: Async;

    fn wallet_address(wallet: &Self::Wallet) -> &Self::Address;
}

pub type WalletOf<ChainDriver> = <ChainDriver as HasWalletType>::Wallet;

#[cgp_component {
  name: WalletSignerComponent,
  provider: WalletSignerProvider,
  context: Chain,
}]
pub trait HasWalletSigner: HasWalletType + HasSignerType {
    fn wallet_signer(wallet: &Self::Wallet) -> &Self::Signer;
}
