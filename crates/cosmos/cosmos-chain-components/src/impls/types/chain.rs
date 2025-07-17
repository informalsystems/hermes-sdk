use alloc::sync::Arc;
use core::time::Duration;
use std::str::FromStr;

use cgp::core::error::CanRaiseAsyncError;
use hermes_core::chain_components::traits::{
    EvidenceFieldsGetter, EvidenceFieldsGetterComponent, EvidenceTypeProvider,
    EvidenceTypeProviderComponent, HasClientIdType, HasEvidenceType,
};
use hermes_core::chain_type_components::impls::UseEventsMessageResponse;
use hermes_core::chain_type_components::traits::{
    ChainIdTypeProviderComponent, EventTypeProviderComponent, HeightAdjuster,
    HeightAdjusterComponent, HeightIncrementer, HeightIncrementerComponent,
    HeightTypeProviderComponent, MessageResponseEventsGetterComponent,
    MessageResponseTypeComponent, MessageTypeProviderComponent, TimeTypeComponent,
    TimeoutTypeComponent,
};
use hermes_core::relayer_components::chain::impls::{
    ProvideBytesPacketCommitment, ProvideBytesPacketReceipt, ProvideCommitmentPrefixBytes,
};
use hermes_core::relayer_components::chain::traits::{
    AckCommitmentHashTypeProviderComponent, AcknowledgementTypeProviderComponent,
    BlockHashComponent, BlockTypeComponent, ChainStatusTypeComponent, ChannelEndTypeComponent,
    ChannelIdTypeComponent, ClientIdTypeComponent, CommitmentPrefixTypeComponent,
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeProviderComponent, ConnectionEndTypeComponent, ConnectionIdTypeComponent,
    GenesisHeightGetter, GenesisHeightGetterComponent, HasBlockType, HasChainId, HasHeightType,
    HasMessageType, HasTimeType, HeightFieldComponent, HeightFieldGetter, MessageSizeEstimator,
    MessageSizeEstimatorComponent, OutgoingPacketTypeComponent, PacketCommitmentTypeComponent,
    PacketReceiptTypeComponent, PortIdTypeComponent, ProvideBlockHash, ProvideBlockType,
    ProvideChainStatusType, ProvideChannelEndType, ProvideChannelIdType, ProvideClientIdType,
    ProvideConnectionEndType, ProvideConnectionIdType, ProvideOutgoingPacketType,
    ProvidePortIdType, ProvideSequenceType, ProvideTimeType, ProvideTimeoutType,
    SequenceTypeComponent, TimeMeasurer, TimeMeasurerComponent,
};
use hermes_prelude::*;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId, Sequence,
};
use ibc::primitives::{Signer, Timestamp};
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use prost::{EncodeError, Message};
use tendermint::abci::Event as AbciEvent;
use tendermint::block::{Block, Id as BlockId};
use tendermint::{Hash, Time};
use time::OffsetDateTime;

use crate::traits::CosmosMessage;
use crate::types::{ChainStatus, UseCosmosCommitmentProof};

pub struct ProvideCosmosChainTypes;

delegate_components! {
    ProvideCosmosChainTypes {
        ChainIdTypeProviderComponent:
            UseType<ChainId>,
        HeightTypeProviderComponent:
            UseType<Height>,
        MessageTypeProviderComponent:
            UseType<CosmosMessage>,
        EventTypeProviderComponent:
            UseType<Arc<AbciEvent>>,
        [
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
        ]:
            UseEventsMessageResponse,
        CommitmentPrefixTypeComponent:
            ProvideCommitmentPrefixBytes,
        [
            CommitmentProofTypeProviderComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
        ]:
            UseCosmosCommitmentProof,
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
        later.duration_since(*earlier).ok()
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

#[cgp_provider(EvidenceTypeProviderComponent)]
impl<Chain> EvidenceTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Evidence = Misbehaviour;
}

#[cgp_provider(EvidenceFieldsGetterComponent)]
impl<Chain, Counterparty> EvidenceFieldsGetter<Chain, Counterparty> for ProvideCosmosChainTypes
where
    Chain: HasEvidenceType<Evidence = Misbehaviour>
        + HasClientIdType<Counterparty, ClientId = ClientId>,
{
    #[allow(deprecated)]
    fn evidence_client_id(evidence: &Misbehaviour) -> ClientId {
        ClientId::from_str(evidence.client_id.as_str()).expect("Invalid client ID in evidence")
    }
}
