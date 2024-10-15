use cgp::prelude::*;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_token_transfer_components::types::packet_data::mint::UseIbcTransferMintPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::UseIbcTransferPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::UseIbcTransferUnescrowPayloadData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::types::packet_data::UseMockAnyPayloadData;

define_components! {
    MockPayloadDataTypes {
        AnyApp: UseMockAnyPayloadData,
        IbcTransferApp: UseIbcTransferPayloadData,
        IbcTransferMintApp: UseIbcTransferMintPayloadData,
        IbcTransferUnescrowApp: UseIbcTransferUnescrowPayloadData,
    }
}
