use core::fmt::Display;

use cgp::prelude::*;

#[cgp_component {
  name: BlockTypeComponent,
  provider: ProvideBlockType,
  context: Chain,
}]
pub trait HasBlockType: Async {
    type Block: Async;
}

#[cgp_component {
  name: BlockHashComponent,
  provider: ProvideBlockHash,
  context: Chain,
}]
pub trait HasBlockHash: HasBlockType {
    type BlockHash: Display + Async;

    fn block_hash(block: &Self::Block) -> &Self::BlockHash;
}
