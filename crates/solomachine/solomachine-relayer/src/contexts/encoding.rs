use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_solomachine_chain_components::encoding::components::*;
use hermes_solomachine_chain_components::types::client_state::SolomachineClientState;
use hermes_solomachine_chain_components::types::consensus_state::SolomachineConsensusState;

#[cgp_context(SolomachineEncodingContextComponents: SolomachineEncodingComponents)]
pub struct SolomachineEncoding;

delegate_components! {
    SolomachineEncodingContextComponents {
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
    }
}

pub struct ProvideSolomachineEncoding;

#[cgp_provider(EncodingTypeComponent)]
impl<Chain> ProvideEncodingType<Chain, AsBytes> for ProvideSolomachineEncoding
where
    Chain: Async,
{
    type Encoding = SolomachineEncoding;
}

#[cgp_provider(DefaultEncodingGetterComponent)]
impl<Chain> DefaultEncodingGetter<Chain, AsBytes> for ProvideSolomachineEncoding
where
    Chain: HasEncodingType<AsBytes, Encoding = SolomachineEncoding>,
{
    fn default_encoding() -> &'static SolomachineEncoding {
        &SolomachineEncoding
    }
}

delegate_components! {
    ProvideSolomachineEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

pub trait CanUseSolomachineEncoding:
    CanEncodeAndDecode<ViaProtobuf, SolomachineClientState>
    + CanEncodeAndDecode<ViaAny, SolomachineClientState>
    + CanEncodeAndDecode<ViaAny, SolomachineConsensusState>
    + CanConvertBothWays<Any, SolomachineClientState>
    + CanConvertBothWays<Any, SolomachineConsensusState>
{
}

impl CanUseSolomachineEncoding for SolomachineEncoding {}
