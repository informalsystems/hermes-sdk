use cgp::core::component::WithContext;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::UseDelegatedType;
use cgp::core::types::traits::TypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::nonce::PacketNonceGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::payloads::PacketPayloadsGetterComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;

use crate::impls::error::RaiseDebugString;
use crate::impls::types::MockChainTypes;

define_components! {
    MockChainComponents {
        TypeComponent:
            UseDelegatedType<MockChainTypes>,
        [
            ErrorTypeComponent,
            AppIdTypeComponent,
            ChannelIdTypeComponent,
            PacketNonceTypeComponent,
            PacketTypeComponent,
            PacketHeaderTypeComponent,
            PayloadHeaderTypeComponent,
            PayloadDataTypeComponent,

            PacketChannelIdGetterComponent,
            PacketNonceGetterComponent,
            PacketPayloadsGetterComponent,
        ]:
            WithContext,
        ErrorRaiserComponent:
            RaiseDebugString,
    }
}
