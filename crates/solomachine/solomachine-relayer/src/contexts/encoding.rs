use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_solomachine_chain_components::encoding::components::*;
use hermes_solomachine_chain_components::types::client_state::SolomachineClientState;
use hermes_solomachine_chain_components::types::consensus_state::SolomachineConsensusState;

pub struct SolomachineEncoding;

pub struct SolomachineEncodingComponents2;

impl HasComponents for SolomachineEncoding {
    type Components = SolomachineEncodingComponents2;
}

with_solomachine_encoding_components! {
    | Components | {
        delegate_components! {
            SolomachineEncodingComponents2 {
                Components: SolomachineEncodingComponents,
            }
        }
    }
}

delegate_components! {
    SolomachineEncodingComponents2 {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
    }
}

pub struct ProvideSolomachineEncoding;

impl<Chain> ProvideEncodingType<Chain, AsBytes> for ProvideSolomachineEncoding
where
    Chain: Async,
{
    type Encoding = SolomachineEncoding;
}

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
