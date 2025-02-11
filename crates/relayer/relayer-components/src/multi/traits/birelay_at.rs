use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: BiRelayTypeAtComponent<TagA, TagB>,
  provider: ProvideBiRelayTypeAt,
  context: Setup,
}]
pub trait HasBiRelayTypeAt<TagA, TagB>: Async {
    type BiRelay: Async;
}

pub type BiRelayAt<Context, TagA, TagB> = <Context as HasBiRelayTypeAt<TagA, TagB>>::BiRelay;

#[cgp_provider(BiRelayTypeAtComponent<TagA, TagB>)]
impl<Context, TagA, TagB, Provider, BiRelay> ProvideBiRelayTypeAt<Context, TagA, TagB>
    for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, BiRelayTypeAtComponent<TagA, TagB>, Type = BiRelay>,
    BiRelay: Async,
{
    type BiRelay = BiRelay;
}
