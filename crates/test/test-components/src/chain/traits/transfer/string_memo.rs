use alloc::string::String;

use cgp::prelude::Async;

use crate::chain::traits::types::memo::ProvideMemoType;

pub struct ProvideStringMemoType;

impl<Chain> ProvideMemoType<Chain> for ProvideStringMemoType
where
    Chain: Async,
{
    type Memo = Option<String>;
}
