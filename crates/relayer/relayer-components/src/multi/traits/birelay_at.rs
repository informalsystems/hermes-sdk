use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: BiRelayTypeAtComponent,
  provider: ProvideBiRelayTypeAt,
  context: Setup,
}]
pub trait HasBiRelayTypeAt<TagA, TagB>: Async {
    type BiRelay: Async;
}

pub type BiRelayAt<Context, TagA, TagB> = <Context as HasBiRelayTypeAt<TagA, TagB>>::BiRelay;

impl<Context, TagA, TagB, Provider, BiRelay> ProvideBiRelayTypeAt<Context, TagA, TagB>
    for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, BiRelayTypeAtComponent, Type = BiRelay>,
    BiRelay: Async,
{
    type BiRelay = BiRelay;
}
