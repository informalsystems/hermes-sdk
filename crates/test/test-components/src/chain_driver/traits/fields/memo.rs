use cgp_core::prelude::*;

use crate::chain_driver::traits::types::memo::HasMemoType;

#[derive_component(DefaultMemoGetterComponent, DefaultMemoGetter<ChainDriver>)]
pub trait HasDefaultMemo: HasMemoType {
    fn default_memo(&self) -> Self::Memo;
}
