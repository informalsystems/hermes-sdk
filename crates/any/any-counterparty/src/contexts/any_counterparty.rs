use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::{CanConvert, ConverterComponent};
use hermes_encoding_components::traits::decode::{CanDecode, DecoderComponent};
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode::EncoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeProviderComponent,
};
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldComponent, ConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeProviderComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ConnectionIdTypeComponent, PortIdTypeComponent,
    SequenceTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packet::OutgoingPacketTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimeoutTypeComponent;

use crate::impls::encoding::convert::AnyClientConverterComponents;
use crate::impls::encoding::encode::AnyClientEncoderComponents;
use crate::impls::types::client_state::ProvideAnyClientState;
use crate::impls::types::consensus_state::ProvideAnyConsensusState;
use crate::types::client_state::AnyClientState;
use crate::types::consensus_state::AnyConsensusState;

#[cgp_context(AnyCounterpartyComponents)]
pub struct AnyCounterparty;

delegate_components! {
    AnyCounterpartyComponents {
        EncodingTypeProviderComponent<AsBytes>:
            UseType<AnyClientEncoding>,
        [
            HeightTypeProviderComponent,
            HeightFieldComponent,
            TimeoutTypeComponent,
            ChainIdTypeProviderComponent,
            ClientIdTypeComponent,
            ConnectionIdTypeComponent,
            ChannelIdTypeComponent,
            PortIdTypeComponent,
            SequenceTypeComponent,
            OutgoingPacketTypeComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ]:
            ProvideAnyClientState,
        [
            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,
        ]:
            ProvideAnyConsensusState,
        EncodingGetterComponent<AsBytes>:
            GetDefaultEncoding,
    }
}

pub struct AnyCounterpartyCosmosComponents;

delegate_components! {
    AnyCounterpartyCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]: QueryAndConvertRawClientState,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawConsensusState,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        AnyCounterparty: AnyCounterpartyCosmosComponents,
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl DefaultEncodingGetter<AnyCounterparty, AsBytes> for AnyCounterpartyComponents {
    fn default_encoding() -> &'static AnyClientEncoding {
        &AnyClientEncoding
    }
}

#[cgp_context(AnyClientEncodingComponents)]
pub struct AnyClientEncoding;

delegate_components! {
    AnyClientEncodingComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            UseDelegate<AnyClientEncoderComponents>,
        [
            EncodedTypeComponent,
            SchemaTypeComponent,
            SchemaGetterComponent,
            MutEncoderComponent,
            MutDecoderComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            EncodedLengthGetterComponent,
        ]:
            CosmosClientEncodingComponents::Provider,
        ConverterComponent:
            UseDelegate<AnyClientConverterComponents>,
    }
}

pub trait CanUseAnyClientEncoding:
    CanDecode<ViaProtobuf, TendermintClientState>
    + CanDecode<ViaProtobuf, Any>
    + CanDecode<ViaProtobuf, AnyClientState>
    + CanDecode<ViaProtobuf, AnyConsensusState>
    + CanConvert<Any, AnyClientState>
    + CanConvert<Any, AnyConsensusState>
{
}

impl CanUseAnyClientEncoding for AnyClientEncoding {}
