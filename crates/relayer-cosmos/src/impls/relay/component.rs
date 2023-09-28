use cgp_core::traits::{Async, HasComponents};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use ibc_relayer_components_extra::components::extra::relay::ExtraRelayComponents;

use crate::contexts::relay::CosmosRelay;

pub struct CosmosRelayComponents;

impl<SrcChain, DstChain> HasComponents for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    type Components = ExtraRelayComponents<CosmosRelayComponents>;
}

impl<SrcChain, DstChain> CanUseExtraAutoRelayer for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
}

// use ibc_relayer_all_in_one::all_for_one::relay::AfoRelay;

// pub trait IsAfoRelay: AfoRelay {}

// impl<SrcChain, DstChain> IsAfoRelay for CosmosRelay<SrcChain, DstChain>
// where
//     SrcChain: ChainHandle,
//     DstChain: ChainHandle,
// {
// }
