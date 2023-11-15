use crate::traits::chain::types::memo::HasMemoType;

pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
