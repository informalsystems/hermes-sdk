use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use hermes_relayer_components::chain::types::aliases::{
    ChannelIdOf, HeightOf, MessageOf, PortIdOf, TimestampOf,
};

use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::chain::HasChainType;
use crate::chain_driver::traits::types::memo::HasMemoType;

#[derive_component(IbcTokenTransferMessageBuilderComponent, IbcTokenTransferMessageBuilder<ChainDriver>)]
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
        channel_id: &ChannelIdOf<Self::Chain, CounterpartyDriver::Chain>,
        port_id: &PortIdOf<Self::Chain, CounterpartyDriver::Chain>,
        recipient_address: &CounterpartyDriver::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&HeightOf<Self::Chain>>,
        timeout_time: Option<&TimestampOf<Self::Chain>>,
    ) -> Result<MessageOf<Self::Chain>, Self::Error>;
}
