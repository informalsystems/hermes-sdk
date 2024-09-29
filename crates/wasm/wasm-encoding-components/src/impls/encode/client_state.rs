use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use ibc::core::client::types::Height;

use crate::types::client_state::WasmClientState;

pub struct EncodeWasmClientState;

delegate_components! {
    EncodeWasmClientState {
        MutEncoderComponent:
            CombineEncoders<HList![
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
            CombineEncoders<HList![
                EncodeByteField<1>,
                EncodeByteField<2>,
                DecodeRequiredProtoField<3, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeWasmClientState {
    type From = HList![Vec<u8>, Vec<u8>, Height];

    type To = WasmClientState;

    fn transform(HList![data, checksum, latest_height]: Self::From) -> Self::To {
        WasmClientState {
            data,
            checksum,
            latest_height,
        }
    }
}
