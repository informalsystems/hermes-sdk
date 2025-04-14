use cgp::prelude::*;
use hermes_encoding_components::impls::{DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;

use crate::types::consensus_state::WasmConsensusState;

pub struct EncodeWasmConsensusState;

delegate_components! {
    EncodeWasmConsensusState {
        MutEncoderComponent:
            EncodeField<
                symbol!("data"),
                EncodeByteField<1>,
            >,
        MutDecoderComponent: DecodeFrom<
            Self,
            EncodeByteField<1>,
        >,
    }
}

impl Transformer for EncodeWasmConsensusState {
    type From = Vec<u8>;

    type To = WasmConsensusState;

    fn transform(data: Vec<u8>) -> WasmConsensusState {
        WasmConsensusState { data }
    }
}
