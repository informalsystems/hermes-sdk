use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::components::client::{
    ConsensusStateFieldComponent, ConsensusStateQuerierComponent, ConsensusStateTypeComponent,
    ConsensusStateWithProofsQuerierComponent, HeightFieldComponent,
};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::traits::convert::{CanConvert, ConverterComponent};
use hermes_encoding_components::traits::decode::{CanDecode, DecoderComponent};
use hermes_encoding_components::traits::encode::EncoderComponent;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, ProvideEncodingType,
};
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::encoding::convert::AnyClientConverterComponents;
use crate::impls::encoding::encode::AnyClientEncoderComponents;
use crate::impls::types::client_state::ProvideAnyClientState;
use crate::impls::types::consensus_state::ProvideAnyConsensusState;
use crate::types::client_state::AnyClientState;
use crate::types::consensus_state::AnyConsensusState;

pub struct AnyCounterparty;

pub struct AnyCounterpartyComponents;

impl HasComponents for AnyCounterparty {
    type Components = AnyCounterpartyComponents;
}

delegate_components! {
    AnyCounterpartyComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideAnyClientState,
        [
            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,
        ]:
            ProvideAnyConsensusState,
        EncodingGetterComponent:
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

impl ProvideEncodingType<AnyCounterparty, AsBytes> for AnyCounterpartyComponents {
    type Encoding = AnyClientEncoding;
}

impl DefaultEncodingGetter<AnyCounterparty, AsBytes> for AnyCounterpartyComponents {
    fn default_encoding() -> &'static AnyClientEncoding {
        &AnyClientEncoding
    }
}

pub struct AnyClientEncoding;

pub struct AnyClientEncodingComponents;

impl HasComponents for AnyClientEncoding {
    type Components = AnyClientEncodingComponents;
}

delegate_components! {
    AnyClientEncodingComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<AnyClientEncoderComponents>,
        [
            EncodedTypeComponent,
            SchemaTypeComponent,
            SchemaGetterComponent,
        ]:
            CosmosEncodingComponents,
        ConverterComponent:
            DelegateEncoding<AnyClientConverterComponents>,
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
