use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_relayer_components::chain::traits::{
    HasChainStatusType, HasChannelIdType, HasOutgoingPacketType, HasPortIdType,
};

use crate::chain::traits::{HasMemoType, HasWalletType};

#[cgp_component {
    provider: TokenIbcTransferrer,
    context: Chain,
}]
#[async_trait]
pub trait CanIbcTransferToken<Counterparty>:
    HasAsyncErrorType
    + HasWalletType
    + HasAmountType
    + HasPortIdType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasOutgoingPacketType<Counterparty>
    + HasMemoType
where
    Counterparty: HasAddressType + HasChainStatusType,
{
    async fn ibc_transfer_token(
        &self,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sender_wallet: &Self::Wallet,
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        counterparty_chain_status: &Counterparty::ChainStatus,
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
