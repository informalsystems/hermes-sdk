#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_encoding_components::traits::convert::ConverterComponent;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
    use hermes_encoding_components::traits::schema::SchemaGetterComponent;
    use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
    use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
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
}
