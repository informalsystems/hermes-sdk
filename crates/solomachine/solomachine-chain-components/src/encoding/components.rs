#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
    use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
    use hermes_encoding_components::traits::convert::ConverterComponent;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::schema::SchemaGetterComponent;
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;

    use crate::encoding::convert::SolomachineConverterComponents;
    use crate::encoding::encoder::SolomachineEncoderComponents;
    use crate::encoding::type_url::SolomachineTypeUrlSchemas;

    cgp_preset! {
        SolomachineEncodingComponents {
            EncodedTypeComponent:
                ProvideEncodedBytes,
            SchemaTypeComponent:
                ProvideStringSchema,
            ConverterComponent:
                UseDelegate<SolomachineConverterComponents>,
            [
                EncoderComponent,
                DecoderComponent,
            ]:
                UseDelegate<SolomachineEncoderComponents>,
            SchemaGetterComponent:
                SolomachineTypeUrlSchemas,
        }
    }
}
