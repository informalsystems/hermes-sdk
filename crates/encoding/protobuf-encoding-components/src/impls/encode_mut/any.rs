use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeFieldWithGetter;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::field::FieldGetter;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use prost_types::Any;

use crate::impls::encode_mut::proto_field::bytes::EncodeByteField;
use crate::impls::encode_mut::proto_field::string::EncodeStringField;

pub struct EncodeAny;

delegate_components! {
    EncodeAny {
        MutEncoderComponent:
            CombineEncoders<HList![
                EncodeFieldWithGetter<
                    Self,
                    symbol!("type_url"),
                    EncodeStringField<1>,
                >,
                EncodeFieldWithGetter<
                    Self,
                    symbol!("value"),
                    EncodeByteField<2>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                EncodeStringField<1>,
                EncodeByteField<2>,
            ]>
        >,
    }
}

impl FieldGetter<Any, symbol!("type_url")> for EncodeAny {
    type Field = String;

    fn get_field(any: &Any, _tag: PhantomData<symbol!("type_url")>) -> &String {
        &any.type_url
    }
}

impl FieldGetter<Any, symbol!("value")> for EncodeAny {
    type Field = Vec<u8>;

    fn get_field(any: &Any, _tag: PhantomData<symbol!("value")>) -> &Vec<u8> {
        &any.value
    }
}

impl Transformer for EncodeAny {
    type From = HList![String, Vec<u8>];

    type To = Any;

    fn transform(HList![type_url, value]: Self::From) -> Self::To {
        Any { type_url, value }
    }
}
