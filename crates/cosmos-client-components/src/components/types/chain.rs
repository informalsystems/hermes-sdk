use alloc::sync::Arc;
use cgp_core::Async;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer_components::chain::traits::types::chain::HasChainTypes;
use ibc_relayer_components::chain::traits::types::chain_id::ChainIdTypeProvider;
use ibc_relayer_components::chain::traits::types::event::EventTypeProvider;
use ibc_relayer_components::chain::traits::types::height::{HasHeightType, HeightTypeProvider};
use ibc_relayer_components::chain::traits::types::ibc::IbcChainTypesProvider;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProvider;
use ibc_relayer_components::chain::traits::types::status::ChainStatusTypeProvider;
use ibc_relayer_components::chain::traits::types::timestamp::{
    HasTimestampType, TimestampTypeProvider,
};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;
use tendermint::abci::Event as AbciEvent;

use crate::traits::message::CosmosMessage;
pub struct ProvideCosmosChainTypes;

impl<Chain> HeightTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Height = Height;
}

impl<Chain> TimestampTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Timestamp = Timestamp;
}

impl<Chain> MessageTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Message = Arc<dyn CosmosMessage>;
}

impl<Chain> EventTypeProvider<Chain> for ProvideCosmosChainTypes
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

impl<Chain> ChainIdTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChainId = ChainId;
}

impl<Chain, Counterparty> IbcChainTypesProvider<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: HasChainTypes,
{
    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;
}
