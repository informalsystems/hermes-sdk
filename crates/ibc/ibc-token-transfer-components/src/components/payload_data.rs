use cgp::prelude::*;

use crate::types::packet_data::mint::UseIbcTransferMintPayloadData;
use crate::types::packet_data::transfer::UseIbcTransferPayloadData;
use crate::types::packet_data::unescrow::UseIbcTransferUnescrowPayloadData;
use crate::types::tags::{IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp};

cgp_preset! {
    IbcTokenTransferPayloadDataTypes {
        IbcTransferApp: UseIbcTransferPayloadData,
        IbcTransferMintApp: UseIbcTransferMintPayloadData,
        IbcTransferUnescrowApp: UseIbcTransferUnescrowPayloadData,
    }
}
