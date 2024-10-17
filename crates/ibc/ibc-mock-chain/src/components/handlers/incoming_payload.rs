use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::dispatch::DispatchMintOrUnescrow;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::mint::HandleIncomingMintTransfer;
use hermes_ibc_token_transfer_components::impls::handlers::incoming::unescrow::HandleIncomingUnescrowTransfer;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::impls::handlers::incoming::HandleIncomingAnyPayloadData;

define_components! {
    MockIncomingPayloadHandlers {
        AnyApp: HandleIncomingAnyPayloadData,
        IbcTransferMintApp: HandleIncomingMintTransfer,
        IbcTransferUnescrowApp: HandleIncomingUnescrowTransfer,
        IbcTransferApp: DispatchMintOrUnescrow<UseContext, UseContext>,
    }
}
