use alloc::vec::Vec;
use cgp_core::Async;

use crate::chain::traits::types::packets::timeout::ProvidePacketReceiptType;

pub struct ProvideBytesPacketReceipt;

impl<Chain, Counterparty> ProvidePacketReceiptType<Chain, Counterparty>
    for ProvideBytesPacketReceipt
where
    Chain: Async,
{
    type PacketReceipt = Vec<u8>;
}
