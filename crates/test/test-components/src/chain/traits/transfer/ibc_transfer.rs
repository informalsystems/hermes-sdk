use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasMemoType;
use crate::chain::traits::types::wallet::HasWalletType;

#[cgp_component {
  provider: TokenIbcTransferrer,
  context: Chain,
}]
#[async_trait]
pub trait CanIbcTransferToken<Counterparty>:
    HasAsyncErrorType
    + HasWalletType
    + HasAmountType
    + HasIbcChainTypes<Counterparty>
    + HasOutgoingPacketType<Counterparty>
    + HasMemoType
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
        memo: &Self::Memo,
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
