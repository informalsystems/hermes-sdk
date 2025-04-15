use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::impls::ProvideCosmosChainTypes;
use hermes_cosmos_chain_components::types::TendermintClientState;
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_encoding_components::impls::GetDefaultEncoding;
use hermes_encoding_components::traits::{
    CanConvert, CanDecode, ConverterComponent, DecodeBufferTypeComponent, DecoderComponent,
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodeBufferTypeComponent,
    EncodedTypeComponent, EncoderComponent, EncodingGetterComponent, EncodingTypeProviderComponent,
    MutDecoderComponent, MutEncoderComponent,
};
pub use hermes_encoding_components::traits::{SchemaGetterComponent, SchemaTypeComponent};
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_protobuf_encoding_components::traits::EncodedLengthGetterComponent;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_relayer_components::chain::impls::{
    QueryAndConvertRawClientState, QueryAndConvertRawConsensusState,
};
use hermes_relayer_components::chain::traits::{
    AllClientStatesQuerierComponent, ChainIdTypeProviderComponent, ChainStatusTypeComponent,
    ChannelIdTypeComponent, ClientIdTypeComponent, ClientStateFieldsComponent,
    ClientStateQuerierComponent, ClientStateTypeComponent, ConnectionIdTypeComponent,
    ConsensusStateFieldComponent, ConsensusStateQuerierComponent, ConsensusStateTypeComponent,
    ConsensusStateWithProofsQuerierComponent, HeightFieldComponent, HeightTypeProviderComponent,
    OutgoingPacketTypeComponent, PortIdTypeComponent, SequenceTypeComponent, TimeoutTypeComponent,
};

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
