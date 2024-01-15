use cgp_core::prelude::*;

use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::wallet::HasWalletType;

#[derive_component(TokenLocalTransferrerComponent, TokenLocalTransferrer<Chain>)]
#[async_trait]
pub trait CanLocalTransferToken:
    HasWalletType + HasAddressType + HasAmountType + HasErrorType
{
    async fn local_transfer_token(
        &self,
        sender_wallet: &Self::Wallet,
        recipient_address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
