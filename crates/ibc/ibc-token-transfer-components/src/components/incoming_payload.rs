use cgp::core::component::UseContext;
use cgp::prelude::*;

use crate::impls::handlers::incoming::dispatch::DispatchMintOrUnescrow;
use crate::impls::handlers::incoming::mint::HandleIncomingMintTransfer;
use crate::impls::handlers::incoming::unescrow::HandleIncomingUnescrowTransfer;
use crate::types::tags::{IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp};

define_components! {
    IbcTransferIncomingPayloadHandlers {
        IbcTransferMintApp: HandleIncomingMintTransfer,
        IbcTransferUnescrowApp: HandleIncomingUnescrowTransfer,
        IbcTransferApp: DispatchMintOrUnescrow<UseContext, UseContext>,
    }
}
