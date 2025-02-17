use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::builders::amount::CanBuildAmount;
use hermes_chain_type_components::traits::fields::amount::denom::HasAmountDenom;
use hermes_chain_type_components::traits::fields::amount::quantity::HasAmountQuantity;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;
use hermes_chain_type_components::traits::types::quantity::HasQuantityType;
use hermes_chain_type_components::traits::types::time::HasTimeType;
use hermes_ibc_components::traits::builders::packet::CanBuildPacket;
use hermes_ibc_components::traits::fields::commitment::proof_height::HasCommitmentProofHeight;
use hermes_ibc_components::traits::fields::message::app_id::HasIbcMessageAppIds;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::packet::header::timeout::HasPacketTimeout;
use hermes_ibc_components::traits::fields::packet::packet::header::HasPacketHeader;
use hermes_ibc_components::traits::fields::packet::packet::nonce::HasPacketNonce;
use hermes_ibc_components::traits::fields::packet::packet::payloads::HasPacketPayloads;
use hermes_ibc_components::traits::fields::payload::app_id::HasPayloadAppIds;
use hermes_ibc_components::traits::fields::payload::data::HasPayloadData;
use hermes_ibc_components::traits::fields::payload::header::HasPayloadHeader;
use hermes_ibc_components::traits::handlers::incoming::packet::CanHandleIncomingPacket;
use hermes_ibc_components::traits::handlers::incoming::payload::CanHandleIncomingPayload;
use hermes_ibc_components::traits::handlers::outgoing::message::CanHandleIbcMessage;
use hermes_ibc_components::traits::handlers::outgoing::packet::CanSendPacket;
use hermes_ibc_components::traits::nonce::CanAllocatePacketNonce;
use hermes_ibc_components::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_ibc_components::traits::queries::recv_packet_commitment::CanQueryHasPacketReceived;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::commitment::path::HasCommitmentPathType;
use hermes_ibc_components::traits::types::commitment::proof::HasCommitmentProofType;
use hermes_ibc_components::traits::types::commitment::value::HasCommitmentValueType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::packet::nonce::HasPacketNonceType;
use hermes_ibc_components::traits::types::packet::packet::HasPacketType;
use hermes_ibc_components::traits::types::packet::timeout::HasPacketTimeoutType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;
use hermes_ibc_components::traits::types::payload::payload::HasPayloadType;
use hermes_ibc_components::types::message_header::IbcMessageHeader;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload::IbcPayload;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;
use hermes_ibc_components::types::tags::apps::any::AnyApp;
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;
use hermes_ibc_token_transfer_components::traits::builders::mint::CanBuildMintPayload;
use hermes_ibc_token_transfer_components::traits::builders::unescrow::CanBuildUnescrowPayload;
use hermes_ibc_token_transfer_components::traits::escrow_registry::escrow::CanRegisterEscrowToken;
use hermes_ibc_token_transfer_components::traits::fields::message::amount::HasMessageTransferAmount;
use hermes_ibc_token_transfer_components::traits::fields::message::receiver::HasMessageTransferReceiver;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::mint_amount::HasPayloadMintAmount;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::receiver::HasIbcTransferReceiver;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::unescrow_amount::HasPayloadUnescrowAmount;
use hermes_ibc_token_transfer_components::traits::mint_registry::lookup_incoming::CanLookupIncomingMintedToken;
use hermes_ibc_token_transfer_components::traits::mint_registry::register::CanRegisterMintedToken;
use hermes_ibc_token_transfer_components::traits::token::create::CanCreateToken;
use hermes_ibc_token_transfer_components::traits::token::transfer::{
    Burn, CanTransferToken, Mint, Unescrow,
};
use hermes_ibc_token_transfer_components::types::message::IbcTransferMessage;
use hermes_ibc_token_transfer_components::types::packet_data::mint::IbcTransferMintPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::IbcTransferUnescrowPayloadData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::client_id::MockClientId;
use crate::types::commitment::path::{
    MockReceivePacketCommitmentPath, MockSendPacketCommitmentPath,
};
use crate::types::commitment::proof::MockCommitmentProof;
use crate::types::denom::MockDenom;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
use crate::types::packet_data::MockAnyPayloadData;
use crate::types::quantity::MockQuantity;
use crate::types::tagged::Tagged;
use crate::types::tags::{ChainA, ChainB};

#[cgp_context(MockChainComponents)]
pub struct MockChain<Chain: Async, Counterparty: Async> {
    /// The current caller of the mock chain methods.
    /// We assume that this is only callable by the mock kernel,
    /// which emulates transaction handling or calling from another contract.
    pub current_caller: Tagged<Chain, Counterparty, MockAddress>,

    /// The current state of the mock chain is a shared pointer to an immutable chain state
    pub current_state: Arc<dyn HasMockChainState<Chain, Counterparty>>,

    /// The pending state of the mock chain is a mutable pointer to the chain state
    pub pending_state: Box<dyn HasMockChainState<Chain, Counterparty>>,

    pub phantom: PhantomData<(Chain, Counterparty)>,
}

pub struct MockChainState<Chain: Async, Counterparty: Async> {
    pub current_height: Tagged<Chain, Counterparty, MockHeight>,
    pub channel_clients: BTreeMap<
        Tagged<Chain, Counterparty, MockChannelId>,
        Tagged<Chain, Counterparty, MockClientId>,
    >,
    pub consensus_states: BTreeMap<
        Tagged<Chain, Counterparty, MockClientId>,
        BTreeMap<Tagged<Counterparty, Chain, MockHeight>, Arc<MockChainState<Counterparty, Chain>>>,
    >,
    pub next_nonce: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
        ),
        Tagged<Chain, Counterparty, MockNonce>,
    >,
    pub received_packets: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
        ),
        BTreeMap<
            Tagged<Counterparty, Chain, MockNonce>,
            IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
        >,
    >,
    pub sent_packets: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
        ),
        BTreeMap<
            Tagged<Chain, Counterparty, MockNonce>,
            IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
        >,
    >,
    pub escrow_balances: BTreeMap<
        (
            Tagged<Chain, Counterparty, MockChannelId>,
            Tagged<Counterparty, Chain, MockChannelId>,
            Tagged<Chain, Counterparty, MockAppId>,
            Tagged<Counterparty, Chain, MockAppId>,
            MockDenom<Chain, Counterparty>,
        ),
        MockQuantity,
    >,
    pub balances: BTreeMap<
        MockDenom<Chain, Counterparty>,
        BTreeMap<Tagged<Chain, Counterparty, MockAddress>, MockQuantity>,
    >,
}

/**
   This is a type wrapper to allow [`MockChain`] to contain fields that refer
   to `Self`, such as [`IbcPacket`], via [`MockChainState`].

   By moving the fields to a separate [`MockChainState`] struct and wrap it inside
   a `dyn HasMockChainState`, we prevent the Rust compiler from overflowing the
   trait resolution when resolving the trait bound of the mock chain fields.
*/
pub trait HasMockChainState<Chain: Async, Counterparty: Async>: Send + Sync + 'static {
    fn mock_chain_state(&self) -> &MockChainState<Chain, Counterparty>;

    fn mock_chain_state_mut(&mut self) -> &mut MockChainState<Chain, Counterparty>;
}

impl<Chain: Async, Counterparty: Async> HasMockChainState<Chain, Counterparty>
    for MockChainState<Chain, Counterparty>
{
    fn mock_chain_state(&self) -> &MockChainState<Chain, Counterparty> {
        self
    }

    fn mock_chain_state_mut(&mut self) -> &mut MockChainState<Chain, Counterparty> {
        self
    }
}

impl<Chain: Async, Counterparty: Async> Clone for MockChainState<Chain, Counterparty> {
    fn clone(&self) -> Self {
        Self {
            current_height: self.current_height.clone(),
            channel_clients: self.channel_clients.clone(),
            consensus_states: self.consensus_states.clone(),
            next_nonce: self.next_nonce.clone(),
            received_packets: self.received_packets.clone(),
            sent_packets: self.sent_packets.clone(),
            escrow_balances: self.escrow_balances.clone(),
            balances: self.balances.clone(),
        }
    }
}

impl<Chain: Async, Counterparty: Async> Default for MockChainState<Chain, Counterparty> {
    fn default() -> Self {
        Self {
            current_height: Default::default(),
            channel_clients: Default::default(),
            consensus_states: Default::default(),
            next_nonce: Default::default(),
            received_packets: Default::default(),
            sent_packets: Default::default(),
            escrow_balances: Default::default(),
            balances: Default::default(),
        }
    }
}

pub type MockChainA = MockChain<ChainA, ChainB>;
pub type MockChainB = MockChain<ChainB, ChainA>;

pub trait CanUseMockChain: HasAsyncErrorType<Error = String>
    + HasHeightType<Height = Tagged<ChainA, ChainB, MockHeight>>
    + HasTimeType<Time = Tagged<ChainA, ChainB, MockHeight>>
    + HasAddressType<Address = Tagged<ChainA, ChainB, MockAddress>>
    + HasDenomType<Denom = MockDenom<ChainA, ChainB>>
    + HasAmountType<Amount = MockAmount<ChainA, ChainB>>
    + HasQuantityType<Quantity = MockQuantity>
    + HasAppIdType<MockChainB, AppId = Tagged<ChainA, ChainB, MockAppId>>
    + HasClientIdType<MockChainB, ClientId = Tagged<ChainA, ChainB, MockClientId>>
    + HasChannelIdType<MockChainB, ChannelId = Tagged<ChainA, ChainB, MockChannelId>>
    + HasPacketTimeoutType<MockChainB, PacketTimeout = Tagged<ChainA, ChainB, MockHeight>>
    + HasPacketNonceType<MockChainB, PacketNonce = Tagged<ChainA, ChainB, MockNonce>>
    + HasPacketType<MockChainB, Packet = IbcPacket<MockChainA, MockChainB>>
    + HasPacketHeaderType<MockChainB, PacketHeader = IbcPacketHeader<MockChainA, MockChainB>>
    + HasPayloadHeaderType<MockChainB, PayloadHeader = IbcPayloadHeader<MockChainA, MockChainB>>
    + HasIbcMessageHeaderType<MockChainB, IbcMessageHeader = IbcMessageHeader<MockChainA, MockChainB>>
    + HasPayloadType<MockChainB, Payload = IbcPayload<MockChainA, MockChainB, AnyApp>>
    + HasPayloadDataType<MockChainB, AnyApp, PayloadData = MockAnyPayloadData<ChainA, ChainB>>
    + HasPayloadDataType<
        MockChainB,
        IbcTransferApp,
        PayloadData = IbcTransferPayloadData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChainB,
        IbcTransferMintApp,
        PayloadData = IbcTransferMintPayloadData<MockChainA, MockChainB>,
    > + HasPayloadDataType<
        MockChainB,
        IbcTransferUnescrowApp,
        PayloadData = IbcTransferUnescrowPayloadData<MockChainA, MockChainB>,
    > + HasIbcMessageType<
        MockChainB,
        IbcTransferApp,
        IbcMessage = IbcTransferMessage<MockChainA, MockChainB>,
    > + HasCommitmentProofType<SendPacket, CommitmentProof = MockCommitmentProof<ChainA, ChainB>>
    + HasCommitmentPathType<SendPacket, CommitmentPath = MockSendPacketCommitmentPath<ChainA, ChainB>>
    + HasCommitmentPathType<
        ReceivePacket,
        CommitmentPath = MockReceivePacketCommitmentPath<ChainA, ChainB>,
    > + HasCommitmentValueType<SendPacket, CommitmentValue = IbcPacket<MockChainA, MockChainB>>
    + HasCommitmentValueType<ReceivePacket, CommitmentValue = IbcPacket<MockChainB, MockChainA>>
    + HasConsensusStateType<MockChainB, ConsensusState = Arc<MockChainState<ChainA, ChainB>>>
    + HasPacketHeader<MockChainB>
    + HasPacketChannelIds<MockChainB>
    + HasPacketPayloads<MockChainB>
    + HasPacketNonce<MockChainB>
    + HasPacketTimeout<MockChainB>
    + HasPayloadHeader<MockChainB>
    + HasPayloadAppIds<MockChainB>
    + HasPayloadData<MockChainB, AnyApp>
    + HasIbcMessageAppIds<MockChainB>
    + HasCommitmentProofHeight<SendPacket>
    + HasCommitmentProofHeight<ReceivePacket>
    + HasAmountDenom
    + HasAmountQuantity
    + CanBuildAmount
    + CanCreateToken<MockChainB>
    + CanTransferToken<Mint>
    + CanTransferToken<Burn>
    + CanTransferToken<Unescrow>
    + CanTransferToken<Burn>
    + CanLookupIncomingMintedToken<MockChainB>
    + CanRegisterMintedToken<MockChainB>
    + CanRegisterEscrowToken<MockChainB>
    + HasPayloadMintAmount<MockChainB, IbcTransferMintApp>
    + HasIbcTransferReceiver<MockChainB, IbcTransferMintApp>
    + HasPayloadUnescrowAmount<MockChainB, IbcTransferUnescrowApp>
    + CanHandleIncomingPayload<MockChainB, IbcTransferUnescrowApp>
    + CanHandleIncomingPayload<MockChainB, IbcTransferApp>
    + CanHandleIncomingPayload<MockChainB, AnyApp>
    + CanHandleIncomingPacket<MockChainB>
    + CanQueryHasPacketReceived<MockChainB>
    + CanQueryConsensusState<MockChainB>
    + CanAllocatePacketNonce<MockChainB>
    + HasMessageTransferReceiver<MockChainB, IbcTransferApp>
    + HasMessageTransferAmount<MockChainB, IbcTransferApp>
    + CanBuildPacket<MockChainB>
    + CanBuildMintPayload<MockChainB, IbcTransferApp>
    + CanBuildUnescrowPayload<MockChainB, IbcTransferApp>
    + CanSendPacket<MockChainB>
    + CanHandleIncomingPayload<MockChainB, IbcTransferMintApp>
    + CanHandleIbcMessage<MockChainB, IbcTransferApp>
{
}

impl CanUseMockChain for MockChainA {}
