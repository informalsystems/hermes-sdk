use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use cgp_core::{DelegateComponent, ErrorRaiser, HasComponents, ProvideErrorType};
use ibc::clients::ics07_tendermint::client_type;
use ibc::clients::ics07_tendermint::header::Header;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateClient;
use ibc::core::ics04_channel::packet::Packet;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::core::{Msg, ValidationContext};
use ibc::proto::Any;
use ibc::Height;
use ibc_relayer_components::components::default::closures::relay::packet_relayer::CanUseDefaultPacketRelayer;
use ibc_relayer_components::components::default::relay::DefaultRelayComponents;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use ibc_relayer_components::runtime::traits::runtime::ProvideRuntime;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::components::relay::MockCosmosRelayComponents;
use crate::contexts::chain::MockCosmosContext;
use crate::contexts::relay::MockCosmosRelay;
use crate::traits::endpoint::BasecoinEndpoint;
use crate::types::error::Error;
use crate::util::dummy::dummy_signer;

impl<Name, SrcChain, DstChain> DelegateComponent<Name> for MockCosmosRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Delegate = MockCosmosRelayComponents;
}

impl<SrcChain, DstChain> HasComponents for MockCosmosRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Components = MockCosmosRelayComponents;
}

impl<SrcChain, DstChain> CanUseDefaultPacketRelayer for MockCosmosRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
}

impl<SrcChain, DstChain> ProvideErrorType<MockCosmosRelay<SrcChain, DstChain>>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Error = Error;
}

impl<SrcChain, DstChain> ProvideRuntime<MockCosmosRelay<SrcChain, DstChain>>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn runtime(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &TokioRuntimeContext {
        &relay.runtime
    }
}

impl<SrcChain, DstChain> ErrorRaiser<MockCosmosRelay<SrcChain, DstChain>, TokioRuntimeError>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        Error::source(e)
    }
}

impl<SrcChain, DstChain> ErrorRaiser<MockCosmosRelay<SrcChain, DstChain>, Error>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<SrcChain, DstChain> HasRelayChains for MockCosmosRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Packet = Packet;

    type SrcChain = MockCosmosContext<SrcChain>;

    type DstChain = MockCosmosContext<DstChain>;

    fn src_chain(&self) -> &MockCosmosContext<SrcChain> {
        &self.src_chain
    }

    fn dst_chain(&self) -> &MockCosmosContext<DstChain> {
        &self.dst_chain
    }

    fn src_client_id(&self) -> &ClientId {
        self.src_client_id()
    }

    fn dst_client_id(&self) -> &ClientId {
        self.dst_client_id()
    }
}

pub struct MockCosmosBuildUpdateClientMessage;

#[async_trait]
impl<SrcChain, DstChain>
    UpdateClientMessageBuilder<MockCosmosRelay<SrcChain, DstChain>, SourceTarget>
    for MockCosmosBuildUpdateClientMessage
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    async fn build_update_client_messages(
        context: &MockCosmosRelay<SrcChain, DstChain>,
        _target: SourceTarget,
        height: &Height,
    ) -> Result<Vec<Any>, Error> {
        let client_counter = context.dst_chain().ibc_context().client_counter()?;

        let client_id = ClientId::new(client_type(), client_counter)?;

        let client_state = context
            .dst_chain()
            .ibc_context()
            .client_state(&ClientId::default())?;

        let light_block = context.src_chain().get_light_block(height)?;

        let header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: client_state.latest_height,
            trusted_next_validator_set: light_block.next_validators,
        };

        let msg_update_client = MsgUpdateClient {
            client_id,
            client_message: header.into(),
            signer: dummy_signer(),
        };

        Ok(vec![msg_update_client.to_any()])
    }
}

#[async_trait]
impl<SrcChain, DstChain>
    UpdateClientMessageBuilder<MockCosmosRelay<SrcChain, DstChain>, DestinationTarget>
    for MockCosmosBuildUpdateClientMessage
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    async fn build_update_client_messages(
        context: &MockCosmosRelay<SrcChain, DstChain>,
        _target: DestinationTarget,
        height: &Height,
    ) -> Result<Vec<Any>, Error> {
        let client_counter = context.src_chain().ibc_context().client_counter()?;

        let client_id = ClientId::new(client_type(), client_counter)?;

        let client_state = context
            .src_chain()
            .ibc_context()
            .client_state(&ClientId::default())?;

        let light_block = context.dst_chain().get_light_block(height)?;

        let header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: client_state.latest_height,
            trusted_next_validator_set: light_block.next_validators,
        };

        let msg_update_client = MsgUpdateClient {
            client_id,
            client_message: header.into(),
            signer: dummy_signer(),
        };

        Ok(vec![msg_update_client.to_any()])
    }
}

#[async_trait]
impl<SrcChain, DstChain> HasPacketLock for MockCosmosRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type PacketLock<'a> = ();

    async fn try_acquire_packet_lock<'a>(&'a self, _packet: &'a Packet) -> Option<()> {
        Some(())
    }
}
