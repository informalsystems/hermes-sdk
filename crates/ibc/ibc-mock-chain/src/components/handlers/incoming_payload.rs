use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::dispatch::DispatchMintOrUnescrow;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::mint::HandleIncomingMintTransfer;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::unescrow::HandleIncomingUnescrowTransfer;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

define_components! {
    MockIncomingPayloadHandlers {
        IbcTransferMintApp: HandleIncomingMintTransfer,
        IbcTransferUnescrowApp: HandleIncomingUnescrowTransfer,
        IbcTransferApp: DispatchMintOrUnescrow<UseContext, UseContext>,
    }
}
