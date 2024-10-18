use cgp::prelude::*;
use hermes_ibc_token_transfer_components::types::message::UseIbcTransferMessage;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferApp;

define_components! {
    MockIbcMessageTypes {
        IbcTransferApp: UseIbcTransferMessage,
    }
}
