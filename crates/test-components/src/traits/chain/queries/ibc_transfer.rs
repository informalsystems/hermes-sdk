use alloc::boxed::Box;
use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::wallet::HasWalletType;

#[derive_component(TokenIbcTransferrerComponent, TokenIbcTransferrer<Chain>)]
#[async_trait]
pub trait CanIbcTransferToken<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasWalletType + HasAmountType + HasErrorType
where
    Counterparty: HasAddressType,
{
    async fn ibc_transfer_token(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sender_wallet: &Self::Wallet,
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
