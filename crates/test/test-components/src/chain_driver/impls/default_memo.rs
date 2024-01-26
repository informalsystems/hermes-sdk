use crate::chain_driver::traits::fields::memo::DefaultMemoGetter;
use crate::chain_driver::traits::types::memo::HasMemoType;

pub struct ProvideDefaultMemo;

impl<ChainDriver> DefaultMemoGetter<ChainDriver> for ProvideDefaultMemo
where
    ChainDriver: HasMemoType,
    ChainDriver::Memo: Default,
{
    fn default_memo(_chain_driver: &ChainDriver) -> ChainDriver::Memo {
        Default::default()
    }
}
