use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_protobuf_encoding_components::types::{Any, Protobuf};

use crate::encoding::components::*;
use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;

pub struct SolomachineEncoding;

pub struct SolomachineEncodingComponents2;

impl HasComponents for SolomachineEncoding {
    type Components = SolomachineEncodingComponents2;
}

with_solomachine_encoding_components! {
    delegate_components! {
        SolomachineEncodingComponents2 {
            @SolomachineEncodingComponents: SolomachineEncodingComponents,
        }
    }
}

delegate_components! {
    SolomachineEncodingComponents2 {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
    }
}

pub struct ProvideSolomachineEncoding;

impl<Chain> ProvideEncodingType<Chain> for ProvideSolomachineEncoding
where
    Chain: Async,
{
    type Encoding = SolomachineEncoding;
}

impl<Chain> DefaultEncodingGetter<Chain> for ProvideSolomachineEncoding
where
    Chain: HasEncodingType<Encoding = SolomachineEncoding>,
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
    CanEncodeAndDecode<Protobuf, SolomachineClientState>
    + CanEncodeAndDecode<Any, SolomachineClientState>
    + CanEncodeAndDecode<Any, SolomachineConsensusState>
    + CanConvertBothWays<Any, SolomachineClientState>
    + CanConvertBothWays<Any, SolomachineConsensusState>
{
}

impl CanUseSolomachineEncoding for SolomachineEncoding {}
