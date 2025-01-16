use alloc::vec::Vec;
use std::vec;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::{UseField, WithField};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::{HasDstClientId, HasSrcClientId};
use hermes_relayer_components::relay::traits::packet_lock::ProvidePacketLock;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};

use crate::relayer_mock::base::error::Error;
use crate::relayer_mock::base::impls::error::HandleMockError;
use crate::relayer_mock::base::types::height::Height as MockHeight;
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::Packet;
use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::components::relay::MockRelayComponents;
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
        RuntimeTypeComponent: WithType<MockRuntimeContext>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            ChainTypeAtComponent<Src>,
            ChainGetterAtComponent<Src>,
        ]:
            UseField<symbol!("src_chain")>,
        [
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Dst>,
        ]:
            UseField<symbol!("dst_chain")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("src_client_id")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("dst_client_id")>,
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
            context.src_client_id().clone(),
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
            context.dst_client_id().clone(),
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
