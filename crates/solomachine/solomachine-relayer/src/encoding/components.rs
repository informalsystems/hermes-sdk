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
