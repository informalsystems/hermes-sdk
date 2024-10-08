use alloc::vec::Vec;

use cgp::core::Async;

use crate::traits::types::packets::ack::ProvideAcknowledgementType;

pub struct ProvideBytesAcknowlegement;

impl<Chain, Counterparty> ProvideAcknowledgementType<Chain, Counterparty>
    for ProvideBytesAcknowlegement
where
    Chain: Async,
{
    type Acknowledgement = Vec<u8>;
}
