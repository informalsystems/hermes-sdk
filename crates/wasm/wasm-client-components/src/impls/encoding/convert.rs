use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::convert::{ConvertFrom, TryConvertFrom};

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, ProtoWasmClientState): TryConvertFrom,
        (ProtoWasmClientState, WasmClientState): ConvertFrom,
    }
}
