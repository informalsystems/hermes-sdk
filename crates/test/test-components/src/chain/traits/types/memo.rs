use cgp::prelude::*;

#[cgp_component {
  name: MemoTypeComponent,
  provider: ProvideMemoType,
  context: ChainDriver,
}]
pub trait HasMemoType: Async {
    type Memo: Async;
}

#[cgp_component {
  name: DefaultMemoGetterComponent,
  provider: DefaultMemoGetter,
  context: ChainDriver,
}]
pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
