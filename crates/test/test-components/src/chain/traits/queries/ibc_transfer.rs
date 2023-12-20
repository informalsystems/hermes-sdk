use alloc::boxed::Box;
use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::wallet::HasWalletType;

#[derive_component(TokenIbcTransferrerComponent, TokenIbcTransferrer<Chain>)]
#[async_trait]
pub trait CanIbcTransferToken<Counterparty>:
    HasErrorType
    + HasWalletType
    + HasAmountType
    + HasIbcChainTypes<Counterparty>
    + HasIbcPacketTypes<Counterparty>
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
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
