//! A HashMap<Height, ChainState> is used to represent the State of the
//! chain.
//! The current_state is the ChainState at all Heights, up to the latest Height.
//! The consensus_states is a HashMap<ClientId, HashMap<Height, ChainState>>.
//! This is used to check the state of a specific client, at a
//! specific height.
//! Usually the consensus_states would use the root hashes of a Merle Tree,
//! but since the MockChain is used for testing and will have a small number
//! of states, the whole state is used. This avoids needing to implement
//! Merkle Trees and Proofs.

use alloc::string::String;
use core::ops::Deref;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec;

use cgp::core::field::HasField;
use cgp::prelude::*;
use eyre::eyre;

use crate::relayer_mock::base::error::{BaseError, Error};
use crate::relayer_mock::base::types::aliases::{
    ChainState, ChannelId, ClientId, MockTimestamp, PortId, Sequence, StateStore,
};
use crate::relayer_mock::base::types::events::Event;
use crate::relayer_mock::base::types::height::{Height, Height as MockHeight};
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::Packet;
use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::base::types::state::State;
use crate::relayer_mock::util::clock::MockClock;
use crate::relayer_mock::util::mutex::MutexUtil;

#[cgp_context(MockChainComponents)]
#[derive(Clone)]
pub struct MockChainContext {
    pub fields: Arc<MockChainContextFields>,
}

#[derive(HasField)]
pub struct MockChainContextFields {
    pub name: String,
    pub past_chain_states: Arc<Mutex<StateStore>>,
    pub current_height: Arc<Mutex<MockHeight>>,
    pub current_state: Arc<Mutex<ChainState>>,
    pub consensus_states: Arc<Mutex<HashMap<ClientId, StateStore>>>,
    pub channel_to_client: Arc<Mutex<HashMap<ChannelId, ClientId>>>,
    pub runtime: MockRuntimeContext,
}

impl Deref for MockChainContext {
    type Target = MockChainContextFields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl MockChainContext {
    pub fn new(name: String, clock: Arc<MockClock>) -> Self {
        let runtime = MockRuntimeContext::new(clock);
        let chain_state = State::default();
        let initial_state: StateStore =
            HashMap::from([(MockHeight::default(), chain_state.clone())]);

        Self {
            fields: Arc::new(MockChainContextFields {
                name,
                past_chain_states: Arc::new(Mutex::new(initial_state)),
                current_height: Arc::new(Mutex::new(Height(1))),
                current_state: Arc::new(Mutex::new(chain_state)),
                consensus_states: Arc::new(Mutex::new(HashMap::new())),
                channel_to_client: Arc::new(Mutex::new(HashMap::new())),
                runtime,
            }),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn runtime(&self) -> &MockRuntimeContext {
        &self.runtime
    }

    // Get the current height of the chain.
    pub fn get_current_height(&self) -> MockHeight {
        let locked_current_height = self.current_height.acquire_mutex();
        *locked_current_height
    }

    /// Get the current state of the chain.
    pub fn get_current_state(&self) -> State {
        let locked_current_state = self.current_state.acquire_mutex();
        let state = locked_current_state;
        state.clone()
    }

    /// Get the client ID from a channel ID.
    pub fn get_client_from_channel(&self, channel_id: &ChannelId) -> Option<ClientId> {
        let locked_channel_to_client = self.channel_to_client.acquire_mutex();
        locked_channel_to_client.get(channel_id).cloned()
    }

    /// Query the chain state at a given Height. This is used to see which receive and
    /// acknowledgment messages have been processed by the Mock Chain.
    pub fn query_state_at_height(&self, height: MockHeight) -> Result<ChainState, Error> {
        let locked_past_chain_states = self.past_chain_states.acquire_mutex();
        let state = locked_past_chain_states
            .get(&height)
            .ok_or_else(|| BaseError::no_chain_state(self.name().to_string(), height.0))?;
        Ok(state.clone())
    }

    /// Query the consensus state of a Client at a given height.
    /// Refer to the `queryConsensusState` of ICS002.
    pub fn query_consensus_state_at_height(
        &self,
        client_id: ClientId,
        height: MockHeight,
    ) -> Result<ChainState, Error> {
        let locked_consensus_states = self.consensus_states.acquire_mutex();
        let client_consensus_states = locked_consensus_states
            .get(&client_id)
            .ok_or_else(|| BaseError::no_consensus_state(client_id.clone()))?;
        let client_consensus_state = client_consensus_states
            .get(&height)
            .ok_or_else(|| BaseError::no_consensus_state_at_height(client_id, height.0))?;
        Ok(client_consensus_state.clone())
    }

    /// In order to get a client ID from a channel ID, the mapping must be manually
    /// registered for the MockChain.
    pub fn map_channel_to_client(&self, channel_id: ChannelId, client_id: ClientId) {
        let mut locked_channel_to_client = self.channel_to_client.acquire_mutex();
        locked_channel_to_client.insert(channel_id, client_id);
    }

    /// Insert a new Consensus State for a given Client at a given Height.
    ///
    /// Returns an error if there is already a consensus state for the client at the given height
    /// which is different from the given state; a chain is not allowed to have two different
    /// consensus states at the same height.
    ///
    /// This is used for the `updateClient` of ICS002. It is considered an act of misbehaviour
    /// when a chain attempts to update an already-existing consensus state with a different value.
    pub fn insert_consensus_state(
        &self,
        client_id: ClientId,
        height: MockHeight,
        state: ChainState,
    ) -> Result<(), Error> {
        let mut locked_consensus_states = self.consensus_states.acquire_mutex();
        let client_consensus_states = match locked_consensus_states.get(&client_id) {
            Some(ccs) => {
                let mut new_client_consensus_states = ccs.clone();
                match new_client_consensus_states.entry(height) {
                    Entry::Occupied(o) => {
                        // Check if the existing Consensus State at the given height differs
                        // from the one passed.
                        if o.get() != &state {
                            return Err(BaseError::consensus_divergence(client_id, height.0).into());
                        }
                    }
                    Entry::Vacant(_) => {
                        new_client_consensus_states.insert(height, state);
                    }
                };
                new_client_consensus_states
            }
            // If the Client doesn't have any Consensus State, create one with the given
            // state as initial Consensus State.
            None => HashMap::from([(height, state)]),
        };
        // Update the Consensus States of the Chain.
        locked_consensus_states.insert(client_id, client_consensus_states);

        Ok(())
    }

    /// Insert the current_state at Height + 1. This is used to advance the chain's Height by 1
    /// without changing its state.
    pub fn new_block(&self) -> Result<(), Error> {
        // Retrieve the current state
        let current_state = self.get_current_state();

        // Update the current_state of the Chain, which will increase the Height by 1.
        self.update_current_state(current_state)?;

        Ok(())
    }

    /// Sending a packet adds a new ChainState with the sent packet information
    /// at a Height + 1.
    pub fn send_packet(&self, height: Height, packet: Packet) -> Result<(), Error> {
        // Retrieve the current_state and update it with the newly sent packet
        let mut new_state = self.get_current_state();

        new_state.update_sent(
            (
                packet.src_port_id.clone(),
                packet.src_channel_id.clone(),
                packet.sequence,
            ),
            packet,
            height,
        );

        // Update the current_state of the Chain
        self.update_current_state(new_state)?;

        Ok(())
    }

    /// Before receiving a packet, the consensus state is used to verify that the packet
    /// was sent by the source chain. And the packet timeout is verified.
    /// Receiving a packet adds a new ChainState with the received packet information
    /// at a Height + 1.
    pub fn receive_packet(
        &self,
        height: Height,
        packet: Packet,
        mut current_state: State,
    ) -> Result<State, Error> {
        // Verify via the consensus state that the packet was sent by the source chain.
        let client_id = self
            .get_client_from_channel(&packet.dst_channel_id)
            .ok_or_else(|| {
                BaseError::no_client_for_channel(
                    packet.src_channel_id.clone(),
                    self.name().to_string(),
                )
            })?;

        let client_consensus =
            self.query_consensus_state_at_height(client_id, height.increment())?;

        if !client_consensus.check_sent((
            packet.src_port_id.clone(),
            packet.src_channel_id.clone(),
            packet.sequence,
        )) {
            return Err(BaseError::generic(eyre!(
                "chain `{}` got a RecvPacket, but client state doesn't have the packet as sent",
                self.name()
            ))
            .into());
        }

        // Check that the packet is not timed out. Current height < packet timeout height.
        let current_height = self.get_current_height();
        let current_time = self.runtime.get_time();

        if current_state.check_timeout(packet.clone(), current_height, current_time.clone()) {
            return Err(BaseError::timeout_receive(
                self.name().to_string(),
                packet.timeout_height.0,
                current_height.0,
                packet.timeout_timestamp.0,
                current_time.0,
            )
            .into());
        }

        // Update the state with the newly received packet
        // This will not commit the updated state to the chain
        current_state.update_received(
            (
                packet.dst_port_id.clone(),
                packet.dst_channel_id.clone(),
                packet.sequence,
            ),
            packet,
            height,
        );

        Ok(current_state)
    }

    /// Receiving an acknowledgement adds a new ChainState with the received acknowledgement
    /// information at a Height + 1.
    pub fn acknowledge_packet(
        &self,
        height: Height,
        packet: Packet,
        mut current_state: State,
    ) -> Result<State, Error> {
        // Verify that with the consensus state that the packet was received by the destination chain.
        let client_id = self
            .get_client_from_channel(&packet.src_channel_id)
            .ok_or_else(|| {
                BaseError::no_client_for_channel(
                    packet.src_channel_id.clone(),
                    self.name().to_string(),
                )
            })?;

        let client_consensus =
            self.query_consensus_state_at_height(client_id, height.increment())?;

        if !client_consensus.check_received((
            packet.dst_port_id.clone(),
            packet.dst_channel_id.clone(),
            packet.sequence,
        )) {
            return Err(BaseError::generic(eyre!(
                "chain `{}` got a AckPacket, but client state doesn't have the packet as received",
                self.name()
            ))
            .into());
        }

        // Update the current state with the newly received acknowledgement
        current_state.update_acknowledged(
            (
                packet.src_port_id.clone(),
                packet.src_channel_id.clone(),
                packet.sequence,
            ),
            packet,
            height,
        );

        Ok(current_state)
    }

    /// Receiving a timed out packet adds a new ChainState with the timed out packet
    /// information at a Height + 1.
    pub fn timeout_packet(
        &self,
        height: Height,
        packet: Packet,
        current_state: State,
    ) -> Result<State, Error> {
        // Verify that with the consensus state that the packet was not received by the destination chain.
        let client_id = self
            .get_client_from_channel(&packet.src_channel_id)
            .ok_or_else(|| {
                BaseError::no_client_for_channel(
                    packet.src_channel_id.clone(),
                    self.name().to_string(),
                )
            })?;

        let client_consensus =
            self.query_consensus_state_at_height(client_id, height.increment())?;

        if client_consensus.check_received((
            packet.dst_port_id.clone(),
            packet.dst_channel_id.clone(),
            packet.sequence,
        )) {
            return Err(BaseError::generic(eyre!(
                "chain `{}` got a TimeoutPacket, but client state received the packet as received",
                self.name()
            ))
            .into());
        }

        Ok(current_state)
    }

    pub fn get_received_packet_information(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Option<(Packet, Height)> {
        let state = self.get_current_state();
        state.get_received((port_id, channel_id, sequence)).cloned()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build_send_packet(
        &self,
        src_channel_id: String,
        src_port_id: String,
        dst_channel_id: String,
        dst_port_id: String,
        sequence: u128,
        timeout_height: Height,
        timeout_timestamp: MockTimestamp,
    ) -> Packet {
        Packet::new(
            src_channel_id,
            src_port_id,
            dst_channel_id,
            dst_port_id,
            sequence,
            timeout_height,
            timeout_timestamp,
        )
    }

    /// Update the chain's current_state and past_chain_states at the same time to ensure
    /// they are always synchronized. Both the state and runtime clock are incremented by
    /// 1000 milliseconds.
    ///
    /// The MockChain must have one and only one ChainState at every height.
    fn update_current_state(&self, state: State) -> Result<(), Error> {
        let latest_height = self.get_current_height();
        let new_height = latest_height.increment();

        // Update current state
        let mut locked_current_state = self.current_state.acquire_mutex();
        *locked_current_state = state;

        // Update current height.
        let mut locked_current_height = self.current_height.acquire_mutex();
        *locked_current_height = new_height;

        self.runtime
            .clock
            .increment_timestamp(MockTimestamp::default())?;

        // After inserting the new state in the current_state, update the past_chain_states
        // at the given height.
        let mut locked_past_chain_states = self.past_chain_states.acquire_mutex();
        locked_past_chain_states.insert(new_height, locked_current_state.clone());

        Ok(())
    }

    /// If the message is a `SendPacket`, update the received packets,
    /// and add a `RecvPacket` event to the returned array of events.
    /// If the message is an `AckPacket`, update the received acknowledgment
    /// packets.
    /// If the message is an `UpdateClient`, update the consensus state.
    ///
    /// When a RecvPacket and AckPacket are received, verify that the client
    /// state has respectively sent the message and received the message.
    ///
    /// The chain state will only be updated if all messages are processed
    /// successfully.
    pub fn process_messages(&self, messages: Vec<MockMessage>) -> Result<Vec<Vec<Event>>, Error> {
        let mut res = vec![];
        let mut current_state = self.get_current_state();

        for m in messages {
            match m {
                MockMessage::RecvPacket(height, packet) => {
                    current_state = self.receive_packet(height, packet, current_state)?;
                    res.push(vec![Event::WriteAcknowledgment(height)]);
                }
                MockMessage::AckPacket(height, packet) => {
                    current_state = self.acknowledge_packet(height, packet, current_state)?;
                    res.push(vec![]);
                }
                MockMessage::UpdateClient(client_id, height, state) => {
                    self.insert_consensus_state(client_id, height, state)?;
                    res.push(vec![]);
                }
                MockMessage::TimeoutPacket(height, packet) => {
                    current_state = self.timeout_packet(height, packet, current_state)?;
                    res.push(vec![]);
                }
            }
        }

        self.update_current_state(current_state)?;

        Ok(res)
    }
}
