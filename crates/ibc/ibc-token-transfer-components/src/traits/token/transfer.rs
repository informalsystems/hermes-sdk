use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

pub struct Mint;

pub struct Burn;

pub struct Escrow;

pub struct Unescrow;

#[derive_component(TokenTransfererComponent, TokenTransferer<Chain>)]
#[async_trait]
pub trait CanTransferToken<Mode>: HasAddressType + HasAmountType + HasErrorType {
    async fn transfer_token(
        &self,
        target: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
