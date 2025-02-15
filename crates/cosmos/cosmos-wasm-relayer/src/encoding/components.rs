use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_cosmos_chain_components::encoding::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_wasm_encoding_components::components::WasmEncodingComponents;

use crate::encoding::convert::WasmCosmosConverterComponents;
use crate::encoding::encode::WasmCosmosEncoderComponents;
use crate::encoding::encode_mut::WasmCosmosEncodeMutComponents;
use crate::encoding::type_url::WasmCosmosTypeUrlSchemas;

cgp_preset! {
    WasmCosmosEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            WasmEncodingComponents::Provider,
        ConverterComponent:
            UseDelegate<WasmCosmosConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            UseDelegate<WasmCosmosEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            UseDelegate<WasmCosmosEncodeMutComponents>,
        [
            SchemaGetterComponent
        ]:
            UseDelegate<WasmCosmosTypeUrlSchemas>,
    }
}
