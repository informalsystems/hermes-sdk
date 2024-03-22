use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, ProtoWasmClientState): ConvertFrom,
        (ProtoWasmClientState, WasmClientState): TryConvertFrom,
    }
}
