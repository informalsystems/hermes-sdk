#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
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

    use crate::encoding::convert::CosmosConverterComponents;
    use crate::encoding::encode::CosmosEncoderComponents;
    use crate::encoding::type_url::CosmosTypeUrlSchemas;

    cgp_preset! {
        CosmosClientEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                DecodeBufferTypeComponent,
                SchemaTypeComponent,
            ]:
                CosmosEncodingComponents,
            ConverterComponent:
                UseDelegate<CosmosConverterComponents>,
            [
                EncoderComponent,
                DecoderComponent,
            ]:
                UseDelegate<CosmosEncoderComponents>,
            [
                MutEncoderComponent,
                MutDecoderComponent,
                EncodedLengthGetterComponent,
            ]:
                CosmosEncodingComponents,
            SchemaGetterComponent:
                CosmosTypeUrlSchemas,
        }
    }
}
