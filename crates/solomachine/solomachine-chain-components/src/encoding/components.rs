use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decoder::DecoderComponent;
pub use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::encoder::EncoderComponent;
pub use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::convert::SolomachineConverterComponents;
use crate::encoding::encoder::SolomachineEncoderComponents;
use crate::encoding::type_url::SolomachineTypeUrlSchemas;

define_components! {
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
