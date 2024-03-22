use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_client_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_client_components::impls::queries::client_state::CosmosQueryClientStateComponents;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::impls::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::impls::default_encoding::GetDefaultEncoding;
use hermes_relayer_components::encode::impls::delegate::DelegateEncoding;
use hermes_relayer_components::encode::impls::via_identity::Identity;
use hermes_relayer_components::encode::traits::convert::ConverterComponent;
use hermes_relayer_components::encode::traits::decoder::{CanDecode, DecoderComponent};
use hermes_relayer_components::encode::traits::encoded::EncodedTypeComponent;
use hermes_relayer_components::encode::traits::encoder::EncoderComponent;
use hermes_relayer_components::encode::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, ProvideEncodingType,
};
use hermes_relayer_components::encode::traits::schema::{
    SchemaGetterComponent, SchemaTypeComponent,
};
use hermes_relayer_components::encode::types::via::Via;

use crate::any_client::impls::encoding::encode::AnyClientEncoderComponents;
use crate::any_client::impls::types::client_state::ProvideAnyClientState;
use crate::any_client::types::client_state::AnyClientState;

pub struct AnyCounterparty;

pub struct AnyCounterpartyComponents;

impl HasComponents for AnyCounterparty {
    type Components = AnyCounterpartyComponents;
}

delegate_components! {
    AnyCounterpartyComponents {
        [
            HeightTypeComponent,
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
        EncodingGetterComponent:
            GetDefaultEncoding,
    }
}

delegate_components! {
    CosmosQueryClientStateComponents {
        AnyCounterparty: QueryAndDecodeClientStateVia<Identity>
    }
}

impl ProvideEncodingType<AnyCounterparty> for AnyCounterpartyComponents {
    type Encoding = AnyClientEncoding;
}

impl DefaultEncodingGetter<AnyCounterparty> for AnyCounterpartyComponents {
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
            ConverterComponent,
            SchemaGetterComponent,
        ]:
            CosmosEncodingComponents,
    }
}

pub trait CheckAnyClientEncoding:
    CanDecode<TendermintClientState>
    + CanDecode<Any>
    + CanDecode<AnyClientState>
    + CanDecode<Via<Identity, AnyClientState>>
{
}

impl CheckAnyClientEncoding for AnyClientEncoding {}
