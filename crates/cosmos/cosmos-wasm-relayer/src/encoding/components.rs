use cgp::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;

use crate::encoding::convert::WasmCosmosConverterComponents;
use crate::encoding::encode::WasmCosmosEncoderComponents;
use crate::encoding::type_url::WasmCosmosTypeUrlSchemas;

define_components! {
    WasmCosmosEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<WasmCosmosConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<WasmCosmosEncoderComponents>,
        [
            SchemaGetterComponent
        ]:
            DelegateEncoding<WasmCosmosTypeUrlSchemas>,
    }
}
