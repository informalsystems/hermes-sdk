use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use std::vec;

use async_trait::async_trait;
use cgp_core::prelude::*;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use ibc_relayer_components::runtime::traits::runtime::{ProvideRuntime, ProvideRuntimeType};

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

delegate_components!(
    MockRelayComponents;
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
    ]:
        HandleMockError,
);

impl ProvideRuntimeType<MockRelayContext> for MockRelayComponents {
    type Runtime = MockRuntimeContext;
}

impl ProvideRuntime<MockRelayContext> for MockRelayComponents {
    fn runtime(relay: &MockRelayContext) -> &MockRuntimeContext {
        &relay.runtime
    }
}

impl HasRelayChains for MockRelayContext {
    type Packet = PacketKey;

    type SrcChain = MockChainContext;

    type DstChain = MockChainContext;

    fn src_chain_error(e: Error) -> Self::Error {
        e
    }

    fn dst_chain_error(e: Error) -> Self::Error {
        e
    }

    fn src_chain(&self) -> &MockChainContext {
        &self.src_chain
    }

    fn dst_chain(&self) -> &MockChainContext {
        &self.dst_chain
    }

    fn src_client_id(&self) -> &ClientId {
        self.dst_to_src_client()
    }

    fn dst_client_id(&self) -> &ClientId {
        self.src_to_dst_client()
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
