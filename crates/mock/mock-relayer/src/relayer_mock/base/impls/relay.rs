use alloc::string::ToString;
use alloc::vec::Vec;
use std::vec;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_relayer_components::relay::traits::chains::{
    HasRelayClientIds, ProvideRelayChains, RelayClientIdGetter,
};
use hermes_relayer_components::relay::traits::packet_lock::ProvidePacketLock;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

use crate::relayer_mock::base::error::Error;
use crate::relayer_mock::base::impls::error::HandleMockError;
use crate::relayer_mock::base::types::aliases::ClientId;
use crate::relayer_mock::base::types::height::Height as MockHeight;
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::Packet;
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
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
    }
}

impl ProvideRelayChains<MockRelayContext> for MockRelayComponents {
    type SrcChain = MockChainContext;

    type DstChain = MockChainContext;

    fn src_chain(relay: &MockRelayContext) -> &MockChainContext {
        &relay.src_chain
    }

    fn dst_chain(relay: &MockRelayContext) -> &MockChainContext {
        &relay.dst_chain
    }
}

impl RelayClientIdGetter<MockRelayContext> for MockRelayComponents {
    fn src_client_id(relay: &MockRelayContext) -> &ClientId {
        relay.dst_to_src_client()
    }

    fn dst_client_id(relay: &MockRelayContext) -> &ClientId {
        relay.src_to_dst_client()
    }
}

pub struct MockBuildUpdateClientMessage;

impl TargetUpdateClientMessageBuilder<MockRelayContext, SourceTarget>
    for MockBuildUpdateClientMessage
{
    async fn build_target_update_client_messages(
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

impl TargetUpdateClientMessageBuilder<MockRelayContext, DestinationTarget>
    for MockBuildUpdateClientMessage
{
    async fn build_target_update_client_messages(
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

impl ProvidePacketLock<MockRelayContext> for MockRelayComponents {
    type PacketLock<'a> = ();

    async fn try_acquire_packet_lock<'a>(
        _relay: &'a MockRelayContext,
        _packet: &'a Packet,
    ) -> Option<()> {
        Some(())
    }
}
