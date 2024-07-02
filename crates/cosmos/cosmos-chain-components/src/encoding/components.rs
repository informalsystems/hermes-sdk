use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decoder::DecoderComponent;
pub use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::encoder::EncoderComponent;
pub use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::convert::CosmosConverterComponents;
use crate::encoding::encode::CosmosEncoderComponents;
use crate::encoding::type_url::CosmosTypeUrlSchemas;

define_components! {
    CosmosEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<CosmosConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<CosmosEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<CosmosTypeUrlSchemas>,
    }
}
