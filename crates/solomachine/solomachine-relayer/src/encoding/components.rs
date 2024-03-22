use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::DecoderComponent;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::convert::SolomachineConverterComponents;
use crate::encoding::encoder::SolomachineEncoderComponents;
use crate::encoding::type_url::SolomachineTypeUrlSchemas;

pub struct SolomachineEncodingComponents;

delegate_components! {
    #[mark_component(IsSolomachineEncodingComponent)]
    SolomachineEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<SolomachineConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<SolomachineEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<SolomachineTypeUrlSchemas>,
    }
}
