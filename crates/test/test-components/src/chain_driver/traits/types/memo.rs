use cgp_core::prelude::*;

#[derive_component(MemoTypeComponent, ProvideMemoType<ChainDriver>)]
pub trait HasMemoType: Async {
    type Memo: Async;
}
