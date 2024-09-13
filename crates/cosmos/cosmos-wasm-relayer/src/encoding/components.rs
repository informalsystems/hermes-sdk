use cgp::prelude::*;
pub use hermes_cosmos_chain_components::encoding::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;

use crate::encoding::convert::WasmCosmosConverterComponents;
use crate::encoding::encode::WasmCosmosEncoderComponents;
use crate::encoding::encode_mut::WasmCosmosEncodeMutComponents;
use crate::encoding::type_url::WasmCosmosTypeUrlSchemas;

define_components! {
    WasmCosmosEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            WasmEncodingComponents,
        ConverterComponent:
            DelegateEncoding<WasmCosmosConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<WasmCosmosEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<WasmCosmosEncodeMutComponents>,
        [
            SchemaGetterComponent
        ]:
            DelegateEncoding<WasmCosmosTypeUrlSchemas>,
    }
}
