use core::marker::PhantomData;

use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::field::EncodeFieldWithGetter;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;

use crate::types::client_message::WasmClientMessage;

pub struct EncodeWasmClientMessage;

delegate_components! {
    EncodeWasmClientMessage {
        MutEncoderComponent:
            EncodeFieldWithGetter<
                Self,
                symbol!("data"),
                EncodeByteField<1>,
            >,
        MutDecoderComponent: DecodeFrom<
            Self,
            EncodeByteField<1>,
        >,
    }
}

impl FieldGetter<WasmClientMessage, symbol!("data")> for EncodeWasmClientMessage {
    type Value = Vec<u8>;

    fn get_field(message: &WasmClientMessage, _tag: PhantomData<symbol!("data")>) -> &Vec<u8> {
        &message.data
    }
}

impl Transformer for EncodeWasmClientMessage {
    type From = Vec<u8>;

    type To = WasmClientMessage;

    fn transform(data: Self::From) -> Self::To {
        WasmClientMessage { data }
    }
}
