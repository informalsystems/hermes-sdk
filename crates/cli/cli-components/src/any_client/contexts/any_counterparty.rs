use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::via_identity::Identity;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::{CanDecode, DecoderComponent};
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, ProvideEncodingType,
};
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::impls::delegate::queries::client_state::QueryAndDecodeClientStateVia;
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

pub struct AnyCounterpartyCosmosComponents;

delegate_components! {
    AnyCounterpartyCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]: QueryAndDecodeClientStateVia<Identity>,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        AnyCounterparty: AnyCounterpartyCosmosComponents,
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
