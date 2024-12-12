use crate::multi::traits::chain_at::{ChainGetterAt, HasChainTypeAt, ProvideChainTypeAt};
use crate::multi::types::tags::{Dst, Src};

pub trait RelayChainsSelector<Relay>:
    ProvideChainTypeAt<Relay, Src> + ProvideChainTypeAt<Relay, Dst>
    + ChainGetterAt<Relay, Src> + ChainGetterAt<Relay, Dst>
where
    Relay: HasChainTypeAt<Src> + HasChainTypeAt<Dst>
{}

impl<Relay, Selector> RelayChainsSelector<Relay> for Selector
where
    Relay: HasChainTypeAt<Src> + HasChainTypeAt<Dst>,
    Selector: ProvideChainTypeAt<Relay, Src> + ProvideChainTypeAt<Relay, Dst>
        + ChainGetterAt<Relay, Src> + ChainGetterAt<Relay, Dst>
{

}