#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use hermes_core::encoding_components::traits::{
        ConverterComponent, DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };
    use hermes_cosmos_core::protobuf_encoding_components::traits::EncodedLengthGetterComponent;
    use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
    use hermes_prelude::*;

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
