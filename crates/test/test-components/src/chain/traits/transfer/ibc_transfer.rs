use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;

use crate::chain::traits::types::address::HasAddressType;
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
