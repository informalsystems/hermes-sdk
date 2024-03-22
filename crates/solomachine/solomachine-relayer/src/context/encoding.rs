use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::types::via::Via;

use crate::encoding::components::{
    IsSolomachineEncodingComponent,
    SolomachineEncodingComponents as BaseSolomachineEncodingComponents,
};
use crate::types::client_state::SolomachineClientState;

pub struct SolomachineEncoding;

pub struct SolomachineEncodingComponents;

impl HasComponents for SolomachineEncoding {
    type Components = SolomachineEncodingComponents;
}

delegate_all!(
    IsSolomachineEncodingComponent,
    BaseSolomachineEncodingComponents,
    SolomachineEncodingComponents,
);

delegate_components! {
    SolomachineEncodingComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
    }
}

pub trait CanUseSolomachineEncoding:
    CanDecode<SolomachineClientState> + CanDecode<Via<Any, SolomachineClientState>>
{
}

impl CanUseSolomachineEncoding for SolomachineEncoding {}
