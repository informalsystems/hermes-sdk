use alloc::boxed::Box;
use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_components::chain::traits::types::timestamp::HasTimestampType;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasMemoType;

#[async_trait]
pub trait CanBuildIbcTokenTransferMessage<Counterparty>:
    HasErrorType
    + HasMessageType
    + HasAddressType
    + HasAmountType
    + HasMemoType
    + HasHeightType
    + HasTimestampType
    + HasIbcChainTypes<Counterparty>
where
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sender_address: &Self::Address,
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&Self::Height>,
        timeout_time: Option<&Self::Timestamp>,
    ) -> Result<Self::Message, Self::Error>;
}
