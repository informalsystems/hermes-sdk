use alloc::sync::Arc;

use cgp_core::{Async, HasErrorType};
use hermes_relayer_components::chain::traits::types::block::{
    HasBlockType, ProvideBlockHash, ProvideBlockType,
};
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::{HasChainId, ProvideChainIdType};
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetter, HasHeightType, HeightIncrementer, ProvideHeightType,
};
use hermes_relayer_components::chain::traits::types::ibc::ProvideIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProvider;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
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
use tendermint::block::{Block, Id as BlockId};
use tendermint::Hash;

use crate::traits::message::CosmosMessage;
pub struct ProvideCosmosChainTypes;

impl<Chain> ProvideHeightType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Height = Height;
}

impl<Chain> HeightIncrementer<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasErrorType,
{
    fn increment_height(height: &Height) -> Result<Height, Chain::Error> {
        Ok(height.increment())
    }
}

impl<Chain> GenesisHeightGetter<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasChainId<ChainId = ChainId> + HasErrorType,
{
    fn genesis_height(chain: &Chain) -> Height {
        Height::from_tm(1_i64.try_into().unwrap(), chain.chain_id())
    }
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

impl<Chain> ProvideChainStatusType<Chain> for ProvideCosmosChainTypes
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

impl<Chain> ProvideBlockType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Block = (BlockId, Block);
}

impl<Chain> ProvideBlockHash<Chain> for ProvideCosmosChainTypes
where
    Chain: HasBlockType<Block = (BlockId, Block)>,
{
    type BlockHash = Hash;

    fn block_hash((block_id, _): &(BlockId, Block)) -> &Hash {
        &block_id.hash
    }
}
