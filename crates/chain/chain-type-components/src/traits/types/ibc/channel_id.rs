use core::fmt::Debug;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

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
    type ChannelId: Debug + Async;
}

impl<Chain, Counterparty, Provider, ChannelId> ProvideChannelIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ChannelIdTypeComponent, Type = ChannelId>,
    ChannelId: Debug + Async,
{
    type ChannelId = ChannelId;
}
