use alloc::string::String;

use cgp::core::component::{UseDelegate, WithContext, WithProvider};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::{UseDelegatedType, WithType};
use cgp::prelude::*;
use hermes_chain_type_components::traits::builders::amount::AmountBuilderComponent;
use hermes_chain_type_components::traits::fields::amount::denom::AmountDenomGetterComponent;
use hermes_chain_type_components::traits::fields::amount::quantity::AmountQuantityGetterComponent;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_chain_type_components::traits::types::ibc::client_id::ClientIdTypeComponent;
use hermes_chain_type_components::traits::types::quantity::QuantityTypeComponent;
use hermes_chain_type_components::traits::types::time::TimeTypeComponent;
use hermes_ibc_components::components::chain::IncomingPacketHandlerComponent;
use hermes_ibc_components::impls::handlers::incoming::packet::full::FullIncomingPacketHandler;
use hermes_ibc_components::impls::handlers::outgoing::packet::build::AllocateNonceAndBuildPacket;
use hermes_ibc_components::impls::handlers::outgoing::packet::commit::CommitSendPacket;
use hermes_ibc_components::traits::fields::message::app_id::IbcMessageAppIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::timeout::PacketTimeoutGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::header::PacketHeaderGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::nonce::PacketNonceGetterComponent;
use hermes_ibc_components::traits::fields::packet::packet::payloads::PacketPayloadsGetterComponent;
use hermes_ibc_components::traits::fields::payload::app_id::PayloadAppIdGetterComponent;
use hermes_ibc_components::traits::fields::payload::data::PayloadDataGetterComponent;
use hermes_ibc_components::traits::fields::payload::header::PayloadHeaderGetterComponent;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandlerComponent;
use hermes_ibc_components::traits::handlers::outgoing::packet::PacketSenderComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::commitment::path::CommitmentPathTypeComponent;
use hermes_ibc_components::traits::types::commitment::value::CommitmentValueTypeComponent;
use hermes_ibc_components::traits::types::message_header::IbcMessageHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;
use hermes_ibc_components::traits::types::payload::payload::PayloadTypeComponent;
use hermes_ibc_components::types::message_header::UseIbcMessageHeader;
use hermes_ibc_components::types::packet::UseIbcPacket;
use hermes_ibc_components::types::packet_header::UseIbcPacketHeader;
use hermes_ibc_components::types::payload::UseIbcPayload;
use hermes_ibc_components::types::payload_header::UseIbcPayloadHeader;
use hermes_ibc_components::types::tags::apps::any::AnyApp;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::mint_amount::PayloadMintAmountGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::receiver::IbcTransferReceiverGetterComponent;
use hermes_ibc_token_transfer_components::traits::fields::payload_data::unescrow_amount::PayloadUnescrowAmountGetterComponent;

use crate::components::handlers::incoming_payload::MockIncomingPayloadHandlers;
use crate::components::ibc_types::MockIbcChainTypes;
use crate::components::payload_data::MockPayloadDataTypes;
use crate::impls::error::RaiseDebugString;
use crate::impls::tagged::UseTaggedType;
use crate::types::amount::UseMockAmountType;
use crate::types::commitment::path::UseMockCommitmentPath;
use crate::types::commitment::value::UseMockCommitmentValue;
use crate::types::denom::UseMockDenomType;
use crate::types::quantity::MockQuantity;

define_components! {
    MockChainComponents {
        [
            HeightTypeComponent,
            TimeTypeComponent,
            AddressTypeComponent,
            AppIdTypeComponent,
            ClientIdTypeComponent,
            ChannelIdTypeComponent,
            PacketNonceTypeComponent,
            PacketTimeoutTypeComponent,
        ]:
            WithProvider<UseTaggedType<UseDelegatedType<MockIbcChainTypes>>>,
        ErrorTypeComponent:
            WithType<String>,
        QuantityTypeComponent:
            WithType<MockQuantity>,
        PacketTypeComponent:
            UseIbcPacket<AnyApp>,
        PacketHeaderTypeComponent:
            UseIbcPacketHeader,
        PayloadTypeComponent:
            UseIbcPayload<AnyApp>,
        PayloadHeaderTypeComponent:
            UseIbcPayloadHeader,
        IbcMessageHeaderTypeComponent:
            UseIbcMessageHeader,
        CommitmentPathTypeComponent:
            UseMockCommitmentPath,
        CommitmentValueTypeComponent:
            UseMockCommitmentValue,
        DenomTypeComponent:
            UseMockDenomType,
        [
            AmountTypeComponent,
            AmountDenomGetterComponent,
            AmountQuantityGetterComponent,
            AmountBuilderComponent,
        ]:
            UseMockAmountType,
        PayloadDataTypeComponent:
            UseDelegate<MockPayloadDataTypes>,
        [
            PacketHeaderGetterComponent,
            PacketChannelIdGetterComponent,
            PacketNonceGetterComponent,
            PacketTimeoutGetterComponent,
            PacketPayloadsGetterComponent,
            PayloadHeaderGetterComponent,
            PayloadAppIdGetterComponent,
            PayloadDataGetterComponent,
            IbcMessageAppIdGetterComponent,
            IbcTransferReceiverGetterComponent,
            PayloadMintAmountGetterComponent,
            PayloadUnescrowAmountGetterComponent,
        ]:
            WithContext,
        ErrorRaiserComponent:
            RaiseDebugString,
        IncomingPayloadHandlerComponent:
            UseDelegate<MockIncomingPayloadHandlers>,
        IncomingPacketHandlerComponent:
            FullIncomingPacketHandler<AnyApp>,
        PacketSenderComponent:
            AllocateNonceAndBuildPacket,
            // CommitSendPacket<AllocateNonceAndBuildPacket>,
    }
}
