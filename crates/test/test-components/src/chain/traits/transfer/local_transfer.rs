use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};

use crate::chain::traits::HasWalletType;

#[cgp_component {
  provider: TokenLocalTransferrer,
  context: Chain,
}]
#[async_trait]
pub trait CanLocalTransferToken:
    HasWalletType + HasAddressType + HasAmountType + HasAsyncErrorType
{
    async fn local_transfer_token(
        &self,
        sender_wallet: &Self::Wallet,
        recipient_address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
