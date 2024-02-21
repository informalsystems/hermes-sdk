use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasMemoType;

#[derive_component(IbcTokenTransferMessageBuilderComponent, IbcTokenTransferMessageBuilder<ChainDriver>)]
#[async_trait]
pub trait CanBuildIbcTokenTransferMessage<Counterparty>:
    HasErrorType
    + HasAmountType
    + HasMemoType
    + HasMessageType
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
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&Self::Height>,
        timeout_time: Option<&Self::Timestamp>,
    ) -> Result<Self::Message, Self::Error>;
}
