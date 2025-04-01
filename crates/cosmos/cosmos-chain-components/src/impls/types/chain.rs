use alloc::sync::Arc;
use core::time::Duration;

use cgp::core::error::CanRaiseAsyncError;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_chain_type_components::impls::types::message_response::UseEventsMessageResponse;
use hermes_chain_type_components::traits::fields::height::{
    HeightAdjuster, HeightAdjusterComponent, HeightIncrementer, HeightIncrementerComponent,
};
use hermes_chain_type_components::traits::fields::message_response_events::MessageResponseEventsGetterComponent;
use hermes_chain_type_components::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_chain_type_components::traits::types::event::EventTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
use hermes_chain_type_components::traits::types::message::MessageTypeComponent;
use hermes_chain_type_components::traits::types::message_response::MessageResponseTypeComponent;
use hermes_chain_type_components::traits::types::time::TimeTypeComponent;
use hermes_chain_type_components::traits::types::timeout::TimeoutTypeComponent;
use hermes_relayer_components::chain::impls::types::commitment::ProvideBytesPacketCommitment;
use hermes_relayer_components::chain::impls::types::commitment_prefix::ProvideCommitmentPrefixBytes;
use hermes_relayer_components::chain::impls::types::receipt::ProvideBytesPacketReceipt;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent, HasBlockType, ProvideBlockHash, ProvideBlockType,
};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ProvideChannelEndType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ProvideConnectionEndType,
};
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetter, GenesisHeightGetterComponent, HasHeightType, HeightFieldComponent,
    HeightFieldGetter,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ConnectionIdTypeComponent, PortIdTypeComponent,
    ProvideChannelIdType, ProvideClientIdType, ProvideConnectionIdType, ProvidePortIdType,
    ProvideSequenceType, SequenceTypeComponent,
};
use hermes_relayer_components::chain::traits::types::message::{
    HasMessageType, MessageSizeEstimator, MessageSizeEstimatorComponent,
};
use hermes_relayer_components::chain::traits::types::packet::{
    OutgoingPacketTypeComponent, ProvideOutgoingPacketType,
};
use hermes_relayer_components::chain::traits::types::packets::ack::{
    AckCommitmentHashTypeProviderComponent, AcknowledgementTypeProviderComponent,
};
use hermes_relayer_components::chain::traits::types::packets::receive::PacketCommitmentTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
use hermes_relayer_components::chain::traits::types::status::{
    ChainStatusTypeComponent, ProvideChainStatusType,
};
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimeType, ProvideTimeType, ProvideTimeoutType, TimeMeasurer, TimeMeasurerComponent,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId, Sequence,
};
use ibc::primitives::{Signer, Timestamp};
use prost::{EncodeError, Message};
use tendermint::abci::Event as AbciEvent;
use tendermint::block::{Block, Id as BlockId};
use tendermint::{Hash, Time};
use time::OffsetDateTime;

use crate::traits::message::CosmosMessage;
use crate::types::commitment_proof::ProvideCosmosCommitmentProof;
use crate::types::status::ChainStatus;
pub struct ProvideCosmosChainTypes;

delegate_components! {
    ProvideCosmosChainTypes {
        ChainIdTypeProviderComponent:
            UseType<ChainId>,
        HeightTypeComponent:
            UseType<Height>,
        MessageTypeComponent:
            WithType<CosmosMessage>,
        EventTypeComponent:
            WithType<Arc<AbciEvent>>,
        [
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
        ]:
            UseEventsMessageResponse,
        CommitmentPrefixTypeComponent:
            ProvideCommitmentPrefixBytes,
        [
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
        ]:
            ProvideCosmosCommitmentProof,
        PacketCommitmentTypeComponent:
            ProvideBytesPacketCommitment,
        [
            AcknowledgementTypeProviderComponent,
            AckCommitmentHashTypeProviderComponent,
        ]:
            UseType<Vec<u8>>,
        PacketReceiptTypeComponent:
            ProvideBytesPacketReceipt,
    }
}

#[cgp_provider(HeightFieldComponent)]
impl<Chain> HeightFieldGetter<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height>,
{
    fn revision_number(height: &Height) -> u64 {
        height.revision_number()
    }

    fn revision_height(height: &Height) -> u64 {
        height.revision_height()
    }
}

#[cgp_provider(HeightIncrementerComponent)]
impl<Chain> HeightIncrementer<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasAsyncErrorType,
{
    fn increment_height(height: &Height) -> Result<Height, Chain::Error> {
        Ok(height.increment())
    }
}

#[cgp_provider(HeightAdjusterComponent)]
impl<Chain> HeightAdjuster<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + CanRaiseAsyncError<ClientError>,
{
    fn add_height(height: &Height, addition: u64) -> Result<Height, Chain::Error> {
        Ok(height.add(addition))
    }

    fn sub_height(height: &Height, subtraction: u64) -> Result<Height, Chain::Error> {
        height.sub(subtraction).map_err(Chain::raise_error)
    }
}

#[cgp_provider(GenesisHeightGetterComponent)]
impl<Chain> GenesisHeightGetter<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasChainId<ChainId = ChainId> + HasAsyncErrorType,
{
    fn genesis_height(chain: &Chain) -> Height {
        Height::new(chain.chain_id().revision_number(), 1).unwrap()
    }
}

#[cgp_provider(TimeTypeComponent)]
impl<Chain> ProvideTimeType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Time = Time;
}

#[cgp_provider(TimeMeasurerComponent)]
impl<Chain> TimeMeasurer<Chain> for ProvideCosmosChainTypes
where
    Chain: HasTimeType<Time = Time>,
{
    fn duration_since(earlier: &Time, later: &Time) -> Option<Duration> {
        earlier.duration_since(*later).ok()
    }
}

#[cgp_provider(TimeoutTypeComponent)]
impl<Chain> ProvideTimeoutType<Chain> for ProvideCosmosChainTypes
where
    Chain: HasTimeType<Time = Time>,
{
    type Timeout = Timestamp;

    fn has_timed_out(time: &Time, timeout: &Timestamp) -> bool {
        OffsetDateTime::from(*time) > OffsetDateTime::from(*timeout)
    }
}

#[cgp_provider(MessageSizeEstimatorComponent)]
impl<Chain> MessageSizeEstimator<Chain> for ProvideCosmosChainTypes
where
    Chain: HasMessageType<Message = CosmosMessage> + CanRaiseAsyncError<EncodeError>,
{
    fn estimate_message_size(message: &CosmosMessage) -> Result<usize, Chain::Error> {
        let raw = message.message.encode_protobuf(&Signer::from(
            "cosmos000000000000000000000000000000000000000".to_string(),
        ));

        Ok(raw.encoded_len())
    }
}

#[cgp_provider(ChainStatusTypeComponent)]
impl<Chain> ProvideChainStatusType<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasTimeType<Time = Time>,
{
    type ChainStatus = ChainStatus;

    fn chain_status_height(status: &ChainStatus) -> &Height {
        &status.height
    }

    fn chain_status_time(status: &ChainStatus) -> &Time {
        &status.time
    }
}

#[cgp_provider(ClientIdTypeComponent)]
impl<Chain, Counterparty> ProvideClientIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ClientId = ClientId;
}

#[cgp_provider(ConnectionIdTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ConnectionId = ConnectionId;
}

#[cgp_provider(ChannelIdTypeComponent)]
impl<Chain, Counterparty> ProvideChannelIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChannelId = ChannelId;
}

#[cgp_provider(PortIdTypeComponent)]
impl<Chain, Counterparty> ProvidePortIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type PortId = PortId;
}

#[cgp_provider(SequenceTypeComponent)]
impl<Chain, Counterparty> ProvideSequenceType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Sequence = Sequence;
}

#[cgp_provider(OutgoingPacketTypeComponent)]
impl<Chain, Counterparty> ProvideOutgoingPacketType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type OutgoingPacket = Packet;
}

#[cgp_provider(BlockTypeComponent)]
impl<Chain> ProvideBlockType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Block = (BlockId, Block);
}

#[cgp_provider(BlockHashComponent)]
impl<Chain> ProvideBlockHash<Chain> for ProvideCosmosChainTypes
where
    Chain: HasBlockType<Block = (BlockId, Block)>,
{
    type BlockHash = Hash;

    fn block_hash((block_id, _): &(BlockId, Block)) -> &Hash {
        &block_id.hash
    }
}

#[cgp_provider(ConnectionEndTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionEndType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ConnectionEnd = ConnectionEnd;
}

#[cgp_provider(ChannelEndTypeComponent)]
impl<Chain, Counterparty> ProvideChannelEndType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChannelEnd = ChannelEnd;
}
