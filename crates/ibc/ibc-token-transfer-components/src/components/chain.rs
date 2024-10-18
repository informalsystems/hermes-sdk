use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandlerComponent;
use hermes_ibc_components::traits::types::message::IbcMessageTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;

use crate::components::incoming_payload::IbcTransferIncomingPayloadHandlers;
use crate::components::payload_data::IbcTokenTransferPayloadDataTypes;
use crate::types::message::UseIbcTransferMessage;

define_components! {
    IbcTokenTransferChainComponents {
        PayloadDataTypeComponent:
            UseDelegate<IbcTokenTransferPayloadDataTypes>,
        IbcMessageTypeComponent:
            UseIbcTransferMessage,
        IncomingPayloadHandlerComponent:
            UseDelegate<IbcTransferIncomingPayloadHandlers>,
    }
}
