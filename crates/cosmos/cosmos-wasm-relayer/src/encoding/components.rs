use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::DecoderComponent;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::convert::WasmCosmosConverterComponents;
use crate::encoding::encode::WasmCosmosEncoderComponents;
use crate::encoding::type_url::WasmCosmosTypeUrlSchemas;

pub struct WasmCosmosEncodingComponents;

delegate_components! {
    #[mark_component(IsWasmCosmosEncodingComponent)]
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
