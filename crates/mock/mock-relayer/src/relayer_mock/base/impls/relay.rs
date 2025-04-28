use alloc::vec::Vec;
use std::vec;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{UseField, WithField};
use cgp::core::types::WithType;
use hermes_prelude::*;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeProviderAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::{
    DestinationTarget, HasDstClientId, HasSrcClientId, PacketLockComponent, ProvidePacketLock,
    SourceTarget, TargetUpdateClientMessageBuilder, TargetUpdateClientMessageBuilderComponent,
};
use hermes_runtime_components::traits::{RuntimeGetterComponent, RuntimeTypeProviderComponent};

use crate::relayer_mock::base::error::Error;
use crate::relayer_mock::base::impls::error::HandleMockError;
use crate::relayer_mock::base::types::height::Height as MockHeight;
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::Packet;
use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::components::relay::MockRelayComponents;
use crate::relayer_mock::contexts::relay::MockRelayContext;

delegate_components! {
    MockRelayComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
        ]:
            HandleMockError,
        RuntimeTypeProviderComponent: WithType<MockRuntimeContext>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            ChainTypeProviderAtComponent<Src>,
            ChainGetterAtComponent<Src>,
        ]:
            UseField<symbol!("src_chain")>,
        [
            ChainTypeProviderAtComponent<Dst>,
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

#[cgp_provider(TargetUpdateClientMessageBuilderComponent)]
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

#[cgp_provider(TargetUpdateClientMessageBuilderComponent)]
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

#[cgp_provider(PacketLockComponent)]
impl ProvidePacketLock<MockRelayContext> for MockRelayComponents {
    type PacketLock<'a> = ();

    async fn try_acquire_packet_lock<'a>(
        _relay: &'a MockRelayContext,
        _packet: &'a Packet,
    ) -> Option<()> {
        Some(())
    }
}
