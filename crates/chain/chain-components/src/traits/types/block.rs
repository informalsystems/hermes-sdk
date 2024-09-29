use core::fmt::Display;

use cgp::prelude::*;

#[derive_component(BlockTypeComponent, ProvideBlockType<Chain>)]
pub trait HasBlockType: Async {
    type Block: Async;
}

#[derive_component(BlockHashComponent, ProvideBlockHash<Chain>)]
pub trait HasBlockHash: HasBlockType {
    type BlockHash: Display + Async;

    fn block_hash(block: &Self::Block) -> &Self::BlockHash;
}
