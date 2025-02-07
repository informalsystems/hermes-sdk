use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::packets::ack::{
    AcknowledgementTypeComponent, ProvideAcknowledgementType,
};

pub struct ProvideBytesAcknowlegement;

#[cgp_provider(AcknowledgementTypeComponent)]
impl<Chain, Counterparty> ProvideAcknowledgementType<Chain, Counterparty>
    for ProvideBytesAcknowlegement
where
    Chain: Async,
{
    type Acknowledgement = Vec<u8>;
}
