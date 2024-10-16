use cgp::prelude::*;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::mint::HandleIncomingMintTransfer;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferMintApp;

define_components! {
    MockIncomingPayloadHandlers {
        IbcTransferMintApp: HandleIncomingMintTransfer,
    }
}
