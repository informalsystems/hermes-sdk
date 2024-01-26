use alloc::string::String;
use cgp_core::prelude::Async;

use crate::chain_driver::traits::types::memo::ProvideMemoType;

pub struct ProvideStringMemoType;

impl<ChainDriver> ProvideMemoType<ChainDriver> for ProvideStringMemoType
where
    ChainDriver: Async,
{
    type Memo = Option<String>;
}
