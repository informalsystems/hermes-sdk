use alloc::sync::Arc;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{HasHeightType, ProvideHeightType};
use hermes_relayer_components::chain::traits::types::ibc::ProvideIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProvider;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeProvider;
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimestampType, ProvideTimestampType,
};
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;
use tendermint::abci::Event as AbciEvent;

use crate::traits::message::CosmosMessage;
pub struct ProvideCosmosChainTypes;

impl<Chain> ProvideHeightType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Height = Height;
}

impl<Chain> ProvideTimestampType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Timestamp = Timestamp;
}

impl<Chain> ProvideMessageType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Message = CosmosMessage;
}

impl<Chain> ProvideEventType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Event = Arc<AbciEvent>;
}

impl<Chain> ChainStatusTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasTimestampType<Timestamp = Timestamp>,
{
    type ChainStatus = ChainStatus;

    fn chain_status_height(status: &ChainStatus) -> &Height {
        &status.height
    }

    fn chain_status_timestamp(status: &ChainStatus) -> &Timestamp {
        &status.timestamp
    }
}

impl<Chain> ProvideChainIdType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChainId = ChainId;
}

impl<Chain, Counterparty> ProvideIbcChainTypes<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: HasChainTypes,
{
    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;
}

impl<Chain, Counterparty> IbcPacketTypesProvider<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type IncomingPacket = Packet;

    type OutgoingPacket = Packet;
}
