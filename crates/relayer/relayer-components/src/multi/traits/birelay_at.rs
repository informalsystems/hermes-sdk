use core::marker::PhantomData;

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

#[cgp_component {
    name: BiRelayGetterAtComponent<TagA, TagB>,
    provider: BiRelayGetterAt,
}]
pub trait HasBiRelayAt<TagA, TagB>: HasBiRelayTypeAt<TagA, TagB> {
    fn birelay_at(&self, _tag: PhantomData<(TagA, TagB)>) -> &Self::BiRelay;
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

#[cgp_provider(BiRelayGetterAtComponent<TagA, TagB>)]
impl<Context, TagA, TagB, Provider> BiRelayGetterAt<Context, TagA, TagB> for WithProvider<Provider>
where
    Context: HasBiRelayTypeAt<TagA, TagB>,
    Provider: FieldGetter<Context, BiRelayGetterAtComponent<TagA, TagB>, Value = Context::BiRelay>,
{
    fn birelay_at(context: &Context, _tag: PhantomData<(TagA, TagB)>) -> &Context::BiRelay {
        Provider::get_field(context, PhantomData)
    }
}
