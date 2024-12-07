use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

use crate::types::with_app_provider::WithAppProvider;

#[cgp_component {
  name: PayloadDataTypeComponent,
  provider: ProvidePayloadDataType,
  context: Chain,
}]
pub trait HasPayloadDataType<Counterparty, App>: Async {
    type PayloadData: Async;
}

pub type PayloadDataOf<Chain, Counterparty, App> =
    <Chain as HasPayloadDataType<Counterparty, App>>::PayloadData;

impl<Chain, Counterparty, App, Provider, PayloadData>
    ProvidePayloadDataType<Chain, Counterparty, App> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PayloadDataTypeComponent, Type = PayloadData>,
    PayloadData: Async,
{
    type PayloadData = PayloadData;
}

impl<Chain, Counterparty, App, Provider, PayloadData>
    ProvidePayloadDataType<Chain, Counterparty, App> for WithAppProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, App, Type = PayloadData>,
    PayloadData: Async,
{
    type PayloadData = PayloadData;
}

impl<Chain, Counterparty, App, Components, Delegate>
    ProvidePayloadDataType<Chain, Counterparty, App> for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<App, Delegate = Delegate>,
    Delegate: ProvidePayloadDataType<Chain, Counterparty, App>,
{
    type PayloadData = Delegate::PayloadData;
}
