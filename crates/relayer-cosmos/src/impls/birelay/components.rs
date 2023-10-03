use cgp_core::{Async, HasComponents};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_all_in_one::all_for_one::birelay::AfoBiRelay;
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

pub trait IsAfoBiRelay: AfoBiRelay {}

impl<ChainA, ChainB> IsAfoBiRelay for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
}
