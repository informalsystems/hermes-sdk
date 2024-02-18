use crate::chain::traits::types::memo::{DefaultMemoGetter, HasMemoType};

pub struct ProvideDefaultMemo;

impl<Chain> DefaultMemoGetter<Chain> for ProvideDefaultMemo
where
    Chain: HasMemoType,
    Chain::Memo: Default,
{
    fn default_memo(_chain: &Chain) -> Chain::Memo {
        Default::default()
    }
}
