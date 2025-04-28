use alloc::string::String;

use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::types::{UseDelegatedType, WithType};
use hermes_chain_type_components::traits::{
    AddressTypeProviderComponent, ChannelIdTypeComponent, ClientIdTypeComponent,
    HeightTypeProviderComponent, QuantityTypeComponent, TimeTypeComponent,
};
use hermes_ibc_components::components::chain::IbcChainComponents;
use hermes_ibc_components::traits::builders::packet::PacketBuilderComponent;
use hermes_ibc_components::traits::fields::message::app_id::IbcMessageAppIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::timeout::PacketTimeoutGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::header::PacketHeaderGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::nonce::PacketNonceGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::payloads::PacketPayloadsGetterComponent;
use hermes_ibc_components::traits::fields::payload::app_id::PayloadAppIdGetterComponent;
use hermes_ibc_components::traits::fields::payload::data::PayloadDataGetterComponent;
use hermes_ibc_components::traits::fields::payload::header::PayloadHeaderGetterComponent;
use hermes_ibc_components::traits::handlers::incoming::packet::IncomingPacketHandlerComponent;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandlerComponent;
use hermes_ibc_components::traits::handlers::outgoing::message::IbcMessageHandlerComponent;
use hermes_ibc_components::traits::handlers::outgoing::packet::PacketSenderComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::message::IbcMessageTypeComponent;
use hermes_ibc_components::traits::types::message_header::IbcMessageHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;
use hermes_ibc_components::traits::types::payload::payload::PayloadTypeComponent;
use hermes_ibc_token_transfer_components::components::chain::IbcTokenTransferChainComponents;
use hermes_ibc_token_transfer_components::traits::builders::mint::MintPayloadBuilderComponent;
use hermes_ibc_token_transfer_components::traits::builders::unescrow::UnescrowPayloadBuilderComponent;
use hermes_ibc_token_transfer_components::traits::fields::message::amount::MessageTransferAmountGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::message::receiver::MessageTransferAddressGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::mint_amount::PayloadMintAmountGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::receiver::IbcTransferReceiverGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::unescrow_amount::PayloadUnescrowAmountGetterComponent;
use hermes_prelude::*;

use crate::components::handlers::incoming_payload::MockPayloadHandlers;
use crate::components::handlers::outgoing_message::MockIbcMessageHandlers;
use crate::components::ibc_message::MockIbcMessageTypes;
use crate::components::ibc_types::MockIbcChainTypes;
use crate::components::payload_data::MockPayloadDataTypes;
pub use crate::contexts::chain::MockChainComponents;
use crate::impls::error::RaiseDebugString;
use crate::impls::tagged::UseTaggedType;
use crate::types::quantity::MockQuantity;

delegate_components! {
    MockChainComponents {
        [
            HeightTypeProviderComponent,
            TimeTypeComponent,
            AddressTypeProviderComponent,
            AppIdTypeComponent,
            ClientIdTypeComponent,
            ChannelIdTypeComponent,
            PacketNonceTypeComponent,
            PacketTimeoutTypeComponent,
        ]:
            WithProvider<UseTaggedType<UseDelegatedType<MockIbcChainTypes>>>,
        [
            PacketTypeComponent,
            PacketBuilderComponent,
            PacketHeaderTypeComponent,
            PayloadTypeComponent,
            PayloadHeaderTypeComponent,
            IbcMessageHeaderTypeComponent,
            PacketHeaderGetterComponent,
            PacketChannelIdGetterComponent,
            PacketNonceGetterComponent,
            PacketTimeoutGetterComponent,
            PacketPayloadsGetterComponent,
            PayloadHeaderGetterComponent,
            PayloadAppIdGetterComponent,
            PayloadDataGetterComponent,
            IbcMessageAppIdGetterComponent,
            IncomingPacketHandlerComponent,
            PacketSenderComponent,
        ]:
            IbcChainComponents::Provider,
        ErrorTypeProviderComponent:
            WithType<String>,
        QuantityTypeComponent:
            WithType<MockQuantity>,
        PayloadDataTypeComponent:
            UseDelegate<MockPayloadDataTypes>,
        IbcMessageTypeComponent:
            UseDelegate<MockIbcMessageTypes>,
        [
            IbcTransferReceiverGetterComponent,
            PayloadMintAmountGetterComponent,
            PayloadUnescrowAmountGetterComponent,
            MintPayloadBuilderComponent,
            UnescrowPayloadBuilderComponent,
            MessageTransferAddressGetterComponent,
            MessageTransferAmountGetterComponent,
        ]:
            IbcTokenTransferChainComponents,
        ErrorRaiserComponent:
            RaiseDebugString,
        IncomingPayloadHandlerComponent:
            UseDelegate<MockPayloadHandlers>,
        IbcMessageHandlerComponent:
            UseDelegate<MockIbcMessageHandlers>,
    }
}
