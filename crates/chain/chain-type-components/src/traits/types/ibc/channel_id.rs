use core::fmt::{Debug, Display};

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: ChannelIdTypeComponent,
  provider: ProvideChannelIdType,
  context: Chain,
}]
pub trait HasChannelIdType<Counterparty>: Sized + Async {
    /**
       The channel ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ChannelId: Display + Debug + Async;
}

#[cgp_provider(ChannelIdTypeComponent)]
impl<Chain, Counterparty, Provider, ChannelId> ProvideChannelIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ChannelIdTypeComponent, Type = ChannelId>,
    ChannelId: Debug + Display + Async,
{
    type ChannelId = ChannelId;
}
