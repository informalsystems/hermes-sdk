use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::delegate::DelegateEncoding;
use hermes_relayer_components::encode::impls::encoded::ProvideEncodedBytes;
use hermes_relayer_components::encode::impls::schema::ProvideStringSchema;
use hermes_relayer_components::encode::traits::convert::ConverterComponent;
use hermes_relayer_components::encode::traits::decoder::DecoderComponent;
use hermes_relayer_components::encode::traits::encoded::EncodedTypeComponent;
use hermes_relayer_components::encode::traits::encoder::EncoderComponent;
use hermes_relayer_components::encode::traits::schema::{
    SchemaGetterComponent, SchemaTypeComponent,
};

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
