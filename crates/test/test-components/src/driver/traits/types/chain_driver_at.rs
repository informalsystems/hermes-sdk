use crate::driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Twindex;

pub trait HasChainDriverAt<const CHAIN: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<CHAIN> + HasChainTypeAt<COUNTERPARTY>
{
    type ChainDriver: HasChainType<
        Chain = ChainTypeAt<Self, CHAIN>,
        Counterparty = ChainTypeAt<Self, COUNTERPARTY>,
    >;

    fn chain_driver_at(&self, index: Twindex<CHAIN, COUNTERPARTY>) -> &Self::ChainDriver;
}
