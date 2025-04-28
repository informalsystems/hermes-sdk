#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use hermes_core::encoding_components::traits::{
        ConverterComponent, DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };
    use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
    use hermes_prelude::*;
    use hermes_protobuf_encoding_components::traits::EncodedLengthGetterComponent;

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
                CosmosEncodingComponents::Provider,
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
                CosmosEncodingComponents::Provider,
            SchemaGetterComponent:
                CosmosTypeUrlSchemas,
        }
    }
}
