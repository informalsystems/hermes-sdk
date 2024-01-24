use cgp_core::prelude::*;

#[derive_component(BlockTypeComponent, ProvideBlockType<Chain>)]
pub trait HasBlockType: Async {
    type Block: Async;
}
