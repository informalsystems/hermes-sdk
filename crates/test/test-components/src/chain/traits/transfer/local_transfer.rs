use cgp::prelude::*;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::wallet::HasWalletType;

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
