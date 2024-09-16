use cgp::prelude::*;
pub use hermes_cosmos_chain_components::encoding::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_protobuf_encoding_components::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
use hermes_protobuf_encoding_components::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;

use crate::impls::encoding::convert::WasmConverterComponents;
use crate::impls::encoding::encode_mut::WasmEncodeMutComponents;
use crate::impls::encoding::encoder::WasmEncoderComponents;
use crate::impls::encoding::type_url::WasmTypeUrlSchemas;

define_components! {
    WasmEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        EncodeBufferTypeComponent:
            ProvideBytesEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideProtoChunksDecodeBuffer,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<WasmConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<WasmEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<WasmEncodeMutComponents>,
        SchemaGetterComponent:
            DelegateEncoding<WasmTypeUrlSchemas>,
    }
}
