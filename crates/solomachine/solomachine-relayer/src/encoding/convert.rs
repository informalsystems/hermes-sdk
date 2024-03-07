use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::convert::{ConvertFrom, TryConvertFrom};

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};

pub struct SolomachineConverterComponents;

delegate_components! {
    SolomachineConverterComponents {
        (SolomachineClientState, ProtoSolomachineClientState): ConvertFrom,
        (ProtoSolomachineClientState, SolomachineClientState): TryConvertFrom,
    }
}
