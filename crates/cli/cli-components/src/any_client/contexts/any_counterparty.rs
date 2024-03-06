use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_cosmos_client_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateFieldsGetterComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::impls::delegate::DelegateEncoding;
use hermes_relayer_components::encode::traits::convert::ConverterComponent;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::traits::decoder::DecoderComponent;
use hermes_relayer_components::encode::traits::encoded::EncodedTypeComponent;
use hermes_relayer_components::encode::traits::encoder::EncoderComponent;
use hermes_relayer_components::encode::traits::has_encoding::EncodingGetter;
use hermes_relayer_components::encode::traits::has_encoding::ProvideEncodingType;
use hermes_relayer_components::encode::traits::schema::SchemaGetterComponent;
use hermes_relayer_components::encode::traits::schema::SchemaTypeComponent;

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
    }
}

impl ProvideEncodingType<AnyCounterparty> for AnyCounterpartyComponents {
    type Encoding = AnyClientEncoding;
}

impl EncodingGetter<AnyCounterparty> for AnyCounterpartyComponents {
    fn encoding(_context: &AnyCounterparty) -> &AnyClientEncoding {
        &AnyClientEncoding
    }
}

#[derive(Default)]
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
    CanDecode<TendermintClientState> + CanDecode<Any> + CanDecode<AnyClientState>
{
}

impl CheckAnyClientEncoding for AnyClientEncoding {}
