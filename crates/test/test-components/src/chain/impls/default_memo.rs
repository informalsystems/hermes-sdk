use cgp::prelude::*;

use crate::chain::traits::types::memo::{
    DefaultMemoGetter, DefaultMemoGetterComponent, HasMemoType,
};

pub struct ProvideDefaultMemo;

#[cgp_provider(DefaultMemoGetterComponent)]
impl<Chain> DefaultMemoGetter<Chain> for ProvideDefaultMemo
where
    Chain: HasMemoType,
    Chain::Memo: Default,
{
    fn default_memo(_chain: &Chain) -> Chain::Memo {
        Default::default()
    }
}
