use alloc::string::ToString;
use alloc::vec::Vec;
use std::vec;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, ProvideRelayChains};
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_runtime_components::traits::runtime::{ProvideRuntime, ProvideRuntimeType};

use crate::relayer_mock::base::error::Error;
use crate::relayer_mock::base::impls::error::HandleMockError;
use crate::relayer_mock::base::types::aliases::ClientId;
use crate::relayer_mock::base::types::height::Height as MockHeight;
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::PacketKey;
use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::components::relay::MockRelayComponents;
use crate::relayer_mock::contexts::chain::MockChainContext;
use crate::relayer_mock::contexts::relay::MockRelayContext;

impl HasComponents for MockRelayContext {
    type Components = MockRelayComponents;
}

delegate_components! {
    MockRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleMockError,
    }
}

impl ProvideRuntimeType<MockRelayContext> for MockRelayComponents {
    type Runtime = MockRuntimeContext;
}

impl ProvideRuntime<MockRelayContext> for MockRelayComponents {
    fn runtime(relay: &MockRelayContext) -> &MockRuntimeContext {
        &relay.runtime
    }
}

impl ProvideRelayChains<MockRelayContext> for MockRelayComponents {
    type Packet = PacketKey;

    type SrcChain = MockChainContext;

    type DstChain = MockChainContext;

    fn src_chain(relay: &MockRelayContext) -> &MockChainContext {
        &relay.src_chain
    }

    fn dst_chain(relay: &MockRelayContext) -> &MockChainContext {
        &relay.dst_chain
    }

    fn src_client_id(relay: &MockRelayContext) -> &ClientId {
        relay.dst_to_src_client()
    }

    fn dst_client_id(relay: &MockRelayContext) -> &ClientId {
        relay.src_to_dst_client()
    }
}

pub struct MockBuildUpdateClientMessage;

#[async_trait]
impl UpdateClientMessageBuilder<MockRelayContext, SourceTarget> for MockBuildUpdateClientMessage {
    async fn build_update_client_messages(
        context: &MockRelayContext,
        _target: SourceTarget,
        height: &MockHeight,
    ) -> Result<Vec<MockMessage>, Error> {
        let state = context.dst_chain.query_state_at_height(*height)?;
        Ok(vec![MockMessage::UpdateClient(
            context.src_client_id().to_string(),
            *height,
            state,
        )])
    }
}

#[async_trait]
impl UpdateClientMessageBuilder<MockRelayContext, DestinationTarget>
    for MockBuildUpdateClientMessage
{
    async fn build_update_client_messages(
        context: &MockRelayContext,
        _target: DestinationTarget,
        height: &MockHeight,
    ) -> Result<Vec<MockMessage>, Error> {
        let state = context.src_chain.query_state_at_height(*height)?;
        Ok(vec![MockMessage::UpdateClient(
            context.dst_client_id().to_string(),
            *height,
            state,
        )])
    }
}

#[async_trait]
impl HasPacketLock for MockRelayContext {
    type PacketLock<'a> = ();

    async fn try_acquire_packet_lock<'a>(&'a self, _packet: &'a PacketKey) -> Option<()> {
        Some(())
    }
}
