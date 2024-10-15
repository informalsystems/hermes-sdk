use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::payload::payload::ProvidePayloadType;

#[derive(HasField)]
pub struct IbcPayload<Chain, Counterparty, App>
where
    Chain: HasPayloadHeaderType<Counterparty> + HasPayloadDataType<Counterparty, App>,
{
    pub header: Chain::PayloadHeader,
    pub data: Chain::PayloadData,
}

pub struct UseIbcPayload<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> ProvidePayloadType<Chain, Counterparty> for UseIbcPayload<App>
where
    Chain: HasPayloadHeaderType<Counterparty> + HasPayloadDataType<Counterparty, App>,
    Counterparty: Async,
    App: Async,
{
    type Payload = IbcPayload<Chain, Counterparty, App>;
}

impl<Chain, Counterparty, App> Clone for IbcPayload<Chain, Counterparty, App>
where
    Chain: HasPayloadHeaderType<Counterparty, PayloadHeader: Clone>
        + HasPayloadDataType<Counterparty, App, PayloadData: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            data: self.data.clone(),
        }
    }
}
