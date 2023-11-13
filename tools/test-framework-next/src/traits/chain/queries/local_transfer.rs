use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::token::HasTokenType;
use crate::traits::chain::types::wallet::HasWalletType;

#[derive_component(TokenLocalTransferrerComponent, TokenLocalTransferrer<Chain>)]
#[async_trait]
pub trait CanLocalTransferToken:
    HasWalletType + HasAddressType + HasTokenType + HasErrorType
{
    async fn local_transfer_token(
        &self,
        sender_wallet: &Self::Wallet,
        recipient_address: &Self::Address,
        token: &Self::Token,
    ) -> Result<(), Self::Error>;
}
