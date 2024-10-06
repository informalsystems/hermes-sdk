use core::marker::PhantomData;

use cgp::core::component::{DelegateTo, WithContext};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::UseDelegatedType;
use cgp::core::types::traits::TypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::fields::message::app_id::IbcMessageAppIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::nonce::PacketNonceGetterComponent;
use hermes_ibc_components::traits::fields::packet::header::timeout::PacketTimeoutGetterComponent;
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

use crate::components::packet_data::MockPacketDataTypes;
use crate::components::types::MockChainTypes;
use crate::impls::error::RaiseDebugString;

pub struct MockChainComponents<Chain, Counterparty>(pub PhantomData<(Chain, Counterparty)>);

delegate_components! {
    <Chain, Counterparty>
    MockChainComponents<Chain, Counterparty> {
        TypeComponent:
            UseDelegatedType<MockChainTypes<Chain, Counterparty>>,
        [
            ErrorTypeComponent,
            AddressTypeComponent,
            DenomTypeComponent,
            AmountTypeComponent,
            AppIdTypeComponent,
            ChannelIdTypeComponent,
            PacketNonceTypeComponent,
            PacketTimeoutTypeComponent,
            PacketTypeComponent,
            PacketHeaderTypeComponent,
            PayloadHeaderTypeComponent,
            IbcMessageHeaderTypeComponent,

            PacketChannelIdGetterComponent,
            PacketNonceGetterComponent,
            PacketTimeoutGetterComponent,
            PacketPayloadsGetterComponent,
            PayloadAppIdGetterComponent,
            IbcMessageAppIdGetterComponent,
        ]:
            WithContext,
        PayloadDataTypeComponent:
            DelegateTo<MockPacketDataTypes>,
        ErrorRaiserComponent:
            RaiseDebugString,
    }
}
