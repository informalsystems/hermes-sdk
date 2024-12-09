use cgp::core::component::{UseDelegate, WithContext};
use cgp::prelude::*;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandlerComponent;
use hermes_ibc_components::traits::types::message::IbcMessageTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;

use crate::components::incoming_payload::IbcTransferIncomingPayloadHandlers;
use crate::components::payload_data::IbcTokenTransferPayloadDataTypes;
use crate::impls::builders::mint::BuildMintPayload;
use crate::impls::builders::unescrow::BuildUnescrowPayload;
use crate::traits::builders::mint::MintPayloadBuilderComponent;
use crate::traits::builders::unescrow::UnescrowPayloadBuilderComponent;
use crate::traits::fields::message::amount::MessageTransferAmountGetterComponent;
use crate::traits::fields::message::receiver::MessageTransferAddressGetterComponent;
use crate::traits::fields::payload_data::mint_amount::PayloadMintAmountGetterComponent;
use crate::traits::fields::payload_data::receiver::IbcTransferReceiverGetterComponent;
use crate::traits::fields::payload_data::unescrow_amount::PayloadUnescrowAmountGetterComponent;
use crate::types::message::UseIbcTransferMessage;

cgp_preset! {
    IbcTokenTransferChainComponents {
        PayloadDataTypeComponent:
            UseDelegate<IbcTokenTransferPayloadDataTypes>,
        IbcMessageTypeComponent:
            UseIbcTransferMessage,
        MintPayloadBuilderComponent:
            BuildMintPayload,
        UnescrowPayloadBuilderComponent:
            BuildUnescrowPayload,
        [
            IbcTransferReceiverGetterComponent,
            PayloadMintAmountGetterComponent,
            PayloadUnescrowAmountGetterComponent,
            MessageTransferAddressGetterComponent,
            MessageTransferAmountGetterComponent,
        ]:
            WithContext,
        IncomingPayloadHandlerComponent:
            UseDelegate<IbcTransferIncomingPayloadHandlers>,
    }
}
