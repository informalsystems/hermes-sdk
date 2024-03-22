use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::DecoderComponent;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::convert::CosmosConverterComponents;
use crate::encoding::encode::CosmosEncoderComponents;
use crate::encoding::type_url::CosmosTypeUrlSchemas;

pub struct CosmosEncodingComponents;

delegate_components! {
    #[mark_component(IsCosmosEncodingComponent)]
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
