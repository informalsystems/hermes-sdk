use alloc::string::String;
use cgp::core::component::{UseDelegate, WithContext, WithProvider};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::{UseDelegatedType, WithType};
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::fields::message::app_id::IbcMessageAppIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::timeout::PacketTimeoutGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::nonce::PacketNonceGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::payloads::PacketPayloadsGetterComponent;
use hermes_ibc_components::traits::fields::payload::app_id::PayloadAppIdGetterComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::message_header::IbcMessageHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;
use hermes_ibc_components::traits::types::payload::payload::PayloadTypeComponent;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::message_header::UseIbcMessageHeader;
use hermes_ibc_components::types::packet::UseIbcPacket;
use hermes_ibc_components::types::packet_header::UseIbcPacketHeader;
use hermes_ibc_components::types::payload::UseIbcPayload;
use hermes_ibc_components::types::payload_header::UseIbcPayloadHeader;

use crate::components::ibc_types::MockIbcChainTypes;
use crate::components::payload_data::MockPayloadDataTypes;
use crate::impls::error::RaiseDebugString;
use crate::impls::tagged::UseTaggedType;

define_components! {
    MockChainComponents {
        [
            HeightTypeComponent,
            AddressTypeComponent,
            DenomTypeComponent,
            AmountTypeComponent,
            AppIdTypeComponent,
            ChannelIdTypeComponent,
            PacketNonceTypeComponent,
            PacketTimeoutTypeComponent,
        ]:
            WithProvider<UseTaggedType<UseDelegatedType<MockIbcChainTypes>>>,
        [
            PacketChannelIdGetterComponent,
            PacketNonceGetterComponent,
            PacketTimeoutGetterComponent,
            PacketPayloadsGetterComponent,
            PayloadAppIdGetterComponent,
            IbcMessageAppIdGetterComponent,
        ]:
            WithContext,
        ErrorTypeComponent:
            WithType<String>,
        PacketTypeComponent:
            UseIbcPacket<AnyApp>,
        PacketHeaderTypeComponent:
            UseIbcPacketHeader,
        PayloadTypeComponent:
            UseIbcPayload<AnyApp>,
        PayloadHeaderTypeComponent:
            UseIbcPayloadHeader,
        PayloadDataTypeComponent:
            UseDelegate<MockPayloadDataTypes>,
        IbcMessageHeaderTypeComponent:
            UseIbcMessageHeader,
        ErrorRaiserComponent:
            RaiseDebugString,
    }
}
