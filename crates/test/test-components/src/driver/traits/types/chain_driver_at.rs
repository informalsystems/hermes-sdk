use crate::driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_at::HasChainTypeAt;
use crate::types::index::Index;

pub trait HasChainDriverAt<const I: usize>: HasChainTypeAt<I> {
    type ChainDriver: HasChainType<Chain = Self::Chain>;

    fn chain_driver_at(&self, index: Index<I>) -> &Self::ChainDriver;
}
