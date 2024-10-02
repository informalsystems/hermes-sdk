use cgp::prelude::*;

#[derive_component(PayloadDataTypeComponent, ProvidePayloadDataType<Chain>)]
pub trait HasPayloadDataType<Counterparty, App>: Async {
    type PayloadData: Async;
}
