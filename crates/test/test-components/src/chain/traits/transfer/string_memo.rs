use alloc::string::String;

use cgp::prelude::*;

use crate::chain::traits::types::memo::{MemoTypeComponent, ProvideMemoType};

pub struct ProvideStringMemoType;

#[cgp_provider(MemoTypeComponent)]
impl<Chain> ProvideMemoType<Chain> for ProvideStringMemoType
where
    Chain: Async,
{
    type Memo = Option<String>;
}
