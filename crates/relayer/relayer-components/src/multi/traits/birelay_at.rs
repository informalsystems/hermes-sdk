use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_type {
    name: BiRelayTypeProviderAtComponent<A, B>,
    provider: BiRelayTypeProviderAt,
}]
pub trait HasBiRelayTypeAt<A, B>: Async {
    type BiRelay: Async;
}

#[cgp_getter {
    name: BiRelayGetterAtComponent<A, B>,
    provider: BiRelayGetterAt,
}]
pub trait HasBiRelayAt<A, B>: HasBiRelayTypeAt<A, B> {
    fn birelay_at(&self) -> &Self::BiRelay;
}

pub type BiRelayAt<Context, TagA, TagB> = <Context as HasBiRelayTypeAt<TagA, TagB>>::BiRelay;
