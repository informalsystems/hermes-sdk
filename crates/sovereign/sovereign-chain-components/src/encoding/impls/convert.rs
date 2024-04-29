use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_wasm_client_components::impls::encoding::convert::WasmConverterComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use sov_celestia_client::types::proto::v1::ClientState as ProtoSovereignClientState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignConverterComponents;

delegate_components! {
    SovereignConverterComponents {
        [
            (WasmClientState, ProtoWasmClientState),
            (ProtoWasmClientState, WasmClientState),
        ]:
            DelegateEncoding<WasmConverterComponents>,
        (SovereignClientState, ProtoSovereignClientState):
            ConvertFrom,
        (ProtoSovereignClientState, SovereignClientState):
            TryConvertFrom,
    }
}
