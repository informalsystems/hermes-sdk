use cgp::prelude::*;
use hermes_ibc_token_transfer_components::impls::handlers::outgoing::message::HandleOutgoingIbcTransfer;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferApp;

define_components! {
    MockIbcMessageHandlers {
        IbcTransferApp: HandleOutgoingIbcTransfer,
    }
}
