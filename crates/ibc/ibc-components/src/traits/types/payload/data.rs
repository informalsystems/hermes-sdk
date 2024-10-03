use cgp::prelude::*;

#[derive_component(PayloadDataTypeComponent, ProvidePayloadDataType<Chain>)]
pub trait HasPayloadDataType<Counterparty, App>: Async {
    type PayloadData: Async;
}

pub type PayloadDataOf<Chain, Counterparty, App> =
    <Chain as HasPayloadDataType<Counterparty, App>>::PayloadData;
