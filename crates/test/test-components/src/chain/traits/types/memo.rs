use cgp::prelude::*;

#[cgp_type]
pub trait HasMemoType: Async {
    type Memo: Async;
}

#[cgp_component {
    provider: DefaultMemoGetter,
}]
pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
