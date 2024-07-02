use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decoder::DecoderComponent;
pub use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::encoder::EncoderComponent;
pub use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

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
