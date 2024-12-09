use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

pub struct Mint;

pub struct Burn;

pub struct Escrow;

pub struct Unescrow;

#[cgp_component {
  provider: TokenTransferer,
  context: Chain,
}]
#[async_trait]
pub trait CanTransferToken<Mode: Async>: HasAddressType + HasAmountType + HasErrorType {
    async fn transfer_token(
        &mut self,
        mode: Mode,
        target: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
