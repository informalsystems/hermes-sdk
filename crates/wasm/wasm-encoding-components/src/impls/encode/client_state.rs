use cgp::core::component::UseContext;
use hermes_core::encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_core::encoding_components::traits::{
    MutDecoderComponent, MutEncoderComponent, Transformer,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeByteField, EncodeLengthDelimitedProtoField,
};
use ibc::core::client::types::Height;

use crate::types::WasmClientState;

pub struct EncodeWasmClientState;

delegate_components! {
    EncodeWasmClientState {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("data"),
                    EncodeByteField<1>,
                >,
                EncodeField<
                    symbol!("checksum"),
                    EncodeByteField<2>,
                >,
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<3, UseContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                EncodeByteField<1>,
                EncodeByteField<2>,
                DecodeRequiredProtoField<3, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeWasmClientState {
    type From = Product![Vec<u8>, Vec<u8>, Height];

    type To = WasmClientState;

    fn transform(product![data, checksum, latest_height]: Self::From) -> Self::To {
        WasmClientState {
            data,
            checksum,
            latest_height,
        }
    }
}
