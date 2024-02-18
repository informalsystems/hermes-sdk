use cgp_core::prelude::*;

#[derive_component(MemoTypeComponent, ProvideMemoType<ChainDriver>)]
pub trait HasMemoType: Async {
    type Memo: Async;
}

#[derive_component(DefaultMemoGetterComponent, DefaultMemoGetter<ChainDriver>)]
pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
