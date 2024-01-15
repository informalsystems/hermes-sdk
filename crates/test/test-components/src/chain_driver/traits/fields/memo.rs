use crate::chain_driver::traits::types::memo::HasMemoType;

pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
