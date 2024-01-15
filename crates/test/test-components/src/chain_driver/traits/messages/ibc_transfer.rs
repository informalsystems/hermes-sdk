use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use hermes_relayer_components::chain::types::aliases::{
    ChannelId, Height, Message, PortId, Timestamp,
};

use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::memo::HasMemoType;
use crate::driver::traits::types::chain::HasChainType;

#[async_trait]
pub trait CanBuildIbcTokenTransferMessage<CounterpartyDriver>:
    HasErrorType + HasChainType + HasAddressType + HasAmountType + HasMemoType
where
    Self::Chain: HasMessageType
        + HasHeightType
        + HasTimestampType
        + HasIbcChainTypes<CounterpartyDriver::Chain>,
    CounterpartyDriver: HasAddressType + HasChainType,
{
    async fn build_ibc_token_transfer_message(
        &self,
        channel_id: &ChannelId<Self::Chain, CounterpartyDriver::Chain>,
        port_id: &PortId<Self::Chain, CounterpartyDriver::Chain>,
        sender_address: &Self::Address,
        recipient_address: &CounterpartyDriver::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&Height<Self::Chain>>,
        timeout_time: Option<&Timestamp<Self::Chain>>,
    ) -> Result<Message<Self::Chain>, Self::Error>;
}
