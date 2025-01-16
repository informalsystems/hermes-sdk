use alloc::sync::Arc;
use core::time::Duration;

use cgp::core::error::CanRaiseAsyncError;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_chain_type_components::impls::types::message_response::UseEventsMessageResponse;
use hermes_chain_type_components::traits::fields::height::HeightIncrementer;
use hermes_chain_type_components::traits::fields::message_response_events::MessageResponseEventsGetterComponent;
use hermes_chain_type_components::traits::types::event::EventTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
use hermes_chain_type_components::traits::types::message::MessageTypeComponent;
use hermes_chain_type_components::traits::types::message_response::MessageResponseTypeComponent;
use hermes_relayer_components::chain::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_relayer_components::chain::impls::types::commitment::ProvideBytesPacketCommitment;
use hermes_relayer_components::chain::impls::types::commitment_prefix::ProvideCommitmentPrefixBytes;
use hermes_relayer_components::chain::impls::types::receipt::ProvideBytesPacketReceipt;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::types::block::{
    HasBlockType, ProvideBlockHash, ProvideBlockType,
};
use hermes_relayer_components::chain::traits::types::chain_id::{HasChainId, ProvideChainIdType};
use hermes_relayer_components::chain::traits::types::channel::ProvideChannelEndType;
use hermes_relayer_components::chain::traits::types::connection::ProvideConnectionEndType;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetter, HasHeightType, HeightFieldGetter,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    ProvideChannelIdType, ProvideClientIdType, ProvideConnectionIdType, ProvidePortIdType,
    ProvideSequenceType,
};
use hermes_relayer_components::chain::traits::types::message::{
    HasMessageType, MessageSizeEstimator,
};
use hermes_relayer_components::chain::traits::types::packet::ProvideOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::PacketCommitmentTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimeType, ProvideTimeType, ProvideTimeoutType, TimeMeasurer,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::channel::types::packet::Packet;
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
        HeightTypeComponent:
            WithType<Height>,
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
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
        PacketReceiptTypeComponent:
            ProvideBytesPacketReceipt,
    }
}

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

impl<Chain> HeightIncrementer<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasAsyncErrorType,
{
    fn increment_height(height: &Height) -> Result<Height, Chain::Error> {
        Ok(height.increment())
    }
}

impl<Chain> GenesisHeightGetter<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasChainId<ChainId = ChainId> + HasAsyncErrorType,
{
    fn genesis_height(chain: &Chain) -> Height {
        Height::new(chain.chain_id().revision_number(), 1).unwrap()
    }
}

impl<Chain> ProvideTimeType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Time = Time;
}

impl<Chain> TimeMeasurer<Chain> for ProvideCosmosChainTypes
where
    Chain: HasTimeType<Time = Time>,
{
    fn duration_since(earlier: &Time, later: &Time) -> Option<Duration> {
        earlier.duration_since(*later).ok()
    }
}

impl<Chain> ProvideTimeoutType<Chain> for ProvideCosmosChainTypes
where
    Chain: HasTimeType<Time = Time>,
{
    type Timeout = Timestamp;

    fn has_timed_out(time: &Time, timeout: &Timestamp) -> bool {
        OffsetDateTime::from(*time) > OffsetDateTime::from(*timeout)
    }
}

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

impl<Chain> ProvideChainIdType<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChainId = ChainId;
}

impl<Chain, Counterparty> ProvideClientIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ClientId = ClientId;
}

impl<Chain, Counterparty> ProvideConnectionIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ConnectionId = ConnectionId;
}

impl<Chain, Counterparty> ProvideChannelIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChannelId = ChannelId;
}

impl<Chain, Counterparty> ProvidePortIdType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type PortId = PortId;
}

impl<Chain, Counterparty> ProvideSequenceType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Sequence = Sequence;
}

impl<Chain, Counterparty> ProvideOutgoingPacketType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
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

impl<Chain, Counterparty> ProvideConnectionEndType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ConnectionEnd = ConnectionEnd;
}

impl<Chain, Counterparty> ProvideChannelEndType<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type ChannelEnd = ChannelEnd;
}
