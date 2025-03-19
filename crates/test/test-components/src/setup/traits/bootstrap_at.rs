use core::marker::PhantomData;

use cgp::prelude::*;

use crate::driver::traits::types::chain_driver::HasChainDriverType;
use crate::driver::traits::types::chain_driver_at::{ChainDriverAt, HasChainDriverTypeAt};

#[cgp_component {
  name: BootstrapAtComponent,
  provider: ProvideBootstrapAt,
  context: Setup,
}]
pub trait HasBootstrapAt<Tag>: HasChainDriverTypeAt<Tag> {
    type Bootstrap: HasChainDriverType<ChainDriver = ChainDriverAt<Self, Tag>>;

    fn chain_bootstrap(&self, _tag: PhantomData<Tag>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, Tag> = <Context as HasBootstrapAt<Tag>>::Bootstrap;
