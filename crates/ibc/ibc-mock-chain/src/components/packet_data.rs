use cgp::prelude::*;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_token_transfer_components::types::packet_data::mint::ProvideIbcTransferMintPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::ProvideIbcTransferPayloadData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::ProvideIbcTransferUnescrowPayloadData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::types::packet_data::ProvideMockAnyPayloadData;

define_components! {
    MockPacketDataTypes {
        AnyApp: ProvideMockAnyPayloadData,
        IbcTransferApp: ProvideIbcTransferPayloadData,
        IbcTransferMintApp: ProvideIbcTransferMintPayloadData,
        IbcTransferUnescrowApp: ProvideIbcTransferUnescrowPayloadData,
    }
}
