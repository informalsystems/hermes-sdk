#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_encoding_components::impls::{ProvideEncodedBytes, ProvideStringSchema};
    use hermes_encoding_components::traits::{
        ConverterComponent, DecoderComponent, EncodedTypeComponent, EncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };

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
