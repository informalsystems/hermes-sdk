use cgp_core::{Async, HasComponents};
use ibc_relayer_components_extra::components::extra::birelay::ExtraBiRelayComponents;

use crate::contexts::birelay::CosmosBiRelay;

pub struct CosmosBiRelayComponents;

impl<ChainA, ChainB> HasComponents for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Components = ExtraBiRelayComponents<CosmosBiRelayComponents>;
}
