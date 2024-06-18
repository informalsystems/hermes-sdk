use alloc::vec::Vec;

use cgp_core::Async;

use crate::chain::traits::types::packets::timeout::ProvidePacketReceiptType;

// TODO: determine if we can use `bool` instead of `Vec<u8>` as the
// canonical `PacketReceipt` type

pub struct ProvideBytesPacketReceipt;

impl<Chain, Counterparty> ProvidePacketReceiptType<Chain, Counterparty>
    for ProvideBytesPacketReceipt
where
    Chain: Async,
{
    type PacketReceipt = Vec<u8>;
}

pub struct ProvideBoolPacketReceipt;

impl<Chain, Counterparty> ProvidePacketReceiptType<Chain, Counterparty> for ProvideBoolPacketReceipt
where
    Chain: Async,
{
    type PacketReceipt = bool;
}
