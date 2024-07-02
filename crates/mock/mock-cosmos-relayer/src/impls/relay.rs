use alloc::vec::Vec;

use cgp_core::component::HasComponents;
use cgp_core::error::{ErrorRaiser, ProvideErrorType};
use hermes_relayer_components::components::default::closures::relay::packet_relayer::CanUseDefaultPacketRelayer;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::relay::traits::packet_lock::ProvidePacketLock;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;
use ibc::clients::tendermint::types::Header;
use ibc::clients::tendermint::TENDERMINT_CLIENT_TYPE;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::context::client_state::ClientStateCommon;
use ibc::core::client::context::ClientValidationContext;
use ibc::core::client::types::msgs::MsgUpdateClient;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ClientId;
use ibc::core::host::ValidationContext;
use ibc::primitives::proto::Any;
use ibc::primitives::ToProto;

use crate::components::relay::MockCosmosRelayComponents;
use crate::contexts::chain::MockCosmosContext;
use crate::contexts::relay::MockCosmosRelay;
use crate::traits::endpoint::BasecoinEndpoint;
use crate::types::error::Error;
use crate::util::dummy::dummy_signer;

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

impl<SrcChain, DstChain> RuntimeGetter<MockCosmosRelay<SrcChain, DstChain>>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn runtime(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &HermesRuntime {
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

impl<SrcChain, DstChain> ProvideRelayChains<MockCosmosRelay<SrcChain, DstChain>>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Packet = Packet;

    type SrcChain = MockCosmosContext<SrcChain>;

    type DstChain = MockCosmosContext<DstChain>;

    fn src_chain(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &MockCosmosContext<SrcChain> {
        &relay.src_chain
    }

    fn dst_chain(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &MockCosmosContext<DstChain> {
        &relay.dst_chain
    }

    fn src_client_id(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &ClientId {
        relay.src_client_id()
    }

    fn dst_client_id(relay: &MockCosmosRelay<SrcChain, DstChain>) -> &ClientId {
        relay.dst_client_id()
    }
}

pub struct MockCosmosBuildUpdateClientMessage;

impl<SrcChain, DstChain>
    TargetUpdateClientMessageBuilder<MockCosmosRelay<SrcChain, DstChain>, SourceTarget>
    for MockCosmosBuildUpdateClientMessage
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    async fn build_target_update_client_messages(
        context: &MockCosmosRelay<SrcChain, DstChain>,
        _target: SourceTarget,
        height: &Height,
    ) -> Result<Vec<Any>, Error> {
        let client_counter = context
            .dst_chain()
            .ibc_context()
            .client_counter()
            .map_err(Error::source)?;

        let client_id =
            ClientId::new(TENDERMINT_CLIENT_TYPE, client_counter).map_err(Error::source)?;

        let client_state = context
            .dst_chain()
            .ibc_context()
            .client_state(&client_id)
            .map_err(Error::source)?;

        let light_block = context.src_chain().get_light_block(height)?;

        let header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: client_state.latest_height(),
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

impl<SrcChain, DstChain>
    TargetUpdateClientMessageBuilder<MockCosmosRelay<SrcChain, DstChain>, DestinationTarget>
    for MockCosmosBuildUpdateClientMessage
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    async fn build_target_update_client_messages(
        context: &MockCosmosRelay<SrcChain, DstChain>,
        _target: DestinationTarget,
        height: &Height,
    ) -> Result<Vec<Any>, Error> {
        let client_counter = context
            .src_chain()
            .ibc_context()
            .client_counter()
            .map_err(Error::source)?;

        let client_id =
            ClientId::new(TENDERMINT_CLIENT_TYPE, client_counter).map_err(Error::source)?;

        let client_state = context
            .src_chain()
            .ibc_context()
            .client_state(&client_id)
            .map_err(Error::source)?;

        let light_block = context.dst_chain().get_light_block(height)?;

        let header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: client_state.latest_height(),
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

impl<SrcChain, DstChain> ProvidePacketLock<MockCosmosRelay<SrcChain, DstChain>>
    for MockCosmosRelayComponents
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type PacketLock<'a> = ();

    async fn try_acquire_packet_lock<'a>(
        _relay: &'a MockCosmosRelay<SrcChain, DstChain>,
        _packet: &'a Packet,
    ) -> Option<()> {
        Some(())
    }
}
