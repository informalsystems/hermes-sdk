use alloc::vec::Vec;

use hermes_prelude::*;

use crate::traits::{PacketReceiptTypeComponent, ProvidePacketReceiptType};

// TODO: determine if we can use `bool` instead of `Vec<u8>` as the
// canonical `PacketReceipt` type

pub struct ProvideBytesPacketReceipt;

#[cgp_provider(PacketReceiptTypeComponent)]
impl<Chain, Counterparty> ProvidePacketReceiptType<Chain, Counterparty>
    for ProvideBytesPacketReceipt
where
    Chain: Async,
{
    type PacketReceipt = Vec<u8>;
}

pub struct ProvideBoolPacketReceipt;

#[cgp_provider(PacketReceiptTypeComponent)]
impl<Chain, Counterparty> ProvidePacketReceiptType<Chain, Counterparty> for ProvideBoolPacketReceipt
where
    Chain: Async,
{
    type PacketReceipt = bool;
}
