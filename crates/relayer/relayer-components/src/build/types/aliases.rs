use alloc::collections::BTreeMap;

use cgp_core::HasErrorType;

use crate::birelay::traits::two_way::{HasTwoChainTypes, HasTwoWayRelayTypes};
use crate::build::traits::birelay::BiRelayOf;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::error::types::ErrorOf;
use crate::relay::traits::chains::HasRelayChains;
use crate::runtime::traits::mutex::MutexOf;
use crate::runtime::traits::runtime::HasRuntimeType;
use crate::runtime::traits::runtime::RuntimeOf;

pub type ChainA<Build> = <BiRelayOf<Build> as HasTwoChainTypes>::ChainA;

pub type ChainB<Build> = <BiRelayOf<Build> as HasTwoChainTypes>::ChainB;

pub type RelayAToB<Build> = <BiRelayOf<Build> as HasTwoWayRelayTypes>::RelayAToB;

pub type RelayBToA<Build> = <BiRelayOf<Build> as HasTwoWayRelayTypes>::RelayBToA;

pub type ChainIdA<Build> = <ChainA<Build> as HasChainIdType>::ChainId;

pub type ChainIdB<Build> = <ChainB<Build> as HasChainIdType>::ChainId;

pub type ClientIdA<Build> = <ChainA<Build> as HasIbcChainTypes<ChainB<Build>>>::ClientId;

pub type ClientIdB<Build> = <ChainB<Build> as HasIbcChainTypes<ChainA<Build>>>::ClientId;

pub type ChainACache<Build> = MutexOf<RuntimeOf<Build>, BTreeMap<ChainIdA<Build>, ChainA<Build>>>;

pub type ChainBCache<Build> = MutexOf<RuntimeOf<Build>, BTreeMap<ChainIdB<Build>, ChainB<Build>>>;

pub type RelayError<Build> = ErrorOf<RelayAToB<Build>>;

pub type RelayAToBCache<Build> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdA<Build>,
            ChainIdB<Build>,
            ClientIdA<Build>,
            ClientIdB<Build>,
        ),
        RelayAToB<Build>,
    >,
>;

pub type RelayBToACache<Build> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdB<Build>,
            ChainIdA<Build>,
            ClientIdB<Build>,
            ClientIdA<Build>,
        ),
        RelayBToA<Build>,
    >,
>;

pub type TargetChain<Build, Target> = <Target as ChainBuildTarget<Build>>::TargetChain;

pub type TargetChainRuntime<Build, Target> =
    <TargetChain<Build, Target> as HasRuntimeType>::Runtime;

pub type TargetChainId<Build, Target> = <TargetChain<Build, Target> as HasChainIdType>::ChainId;

pub type TargetClientId<Build, Target> =
    <TargetChain<Build, Target> as HasIbcChainTypes<CounterpartyChain<Build, Target>>>::ClientId;

pub type CounterpartyChain<Build, Target> = <Target as ChainBuildTarget<Build>>::CounterpartyChain;

pub type CounterpartyChainId<Build, Target> =
    <CounterpartyChain<Build, Target> as HasChainIdType>::ChainId;

pub type CounterpartyClientId<Build, Target> =
    <CounterpartyChain<Build, Target> as HasIbcChainTypes<TargetChain<Build, Target>>>::ClientId;

pub type TargetChainCache<Build, Target> =
    MutexOf<RuntimeOf<Build>, BTreeMap<TargetChainId<Build, Target>, TargetChain<Build, Target>>>;

pub type TargetRelay<Build, Target> = <Target as RelayBuildTarget<Build>>::TargetRelay;

pub type TargetRelayError<Build, Target> = <TargetRelay<Build, Target> as HasErrorType>::Error;

pub type SrcChainTarget<Build, Target> = <Target as RelayBuildTarget<Build>>::SrcChainTarget;

pub type DstChainTarget<Build, Target> = <Target as RelayBuildTarget<Build>>::DstChainTarget;

pub type TargetSrcChain<Build, Target> = <TargetRelay<Build, Target> as HasRelayChains>::SrcChain;

pub type TargetDstChain<Build, Target> = <TargetRelay<Build, Target> as HasRelayChains>::DstChain;

pub type TargetSrcChainId<Build, Target> =
    <TargetSrcChain<Build, Target> as HasChainIdType>::ChainId;

pub type TargetDstChainId<Build, Target> =
    <TargetDstChain<Build, Target> as HasChainIdType>::ChainId;

pub type TargetSrcClientId<Build, Target> =
    <TargetSrcChain<Build, Target> as HasIbcChainTypes<TargetDstChain<Build, Target>>>::ClientId;

pub type TargetDstClientId<Build, Target> =
    <TargetDstChain<Build, Target> as HasIbcChainTypes<TargetSrcChain<Build, Target>>>::ClientId;

pub type TargetRelayCache<Build, Target> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            TargetSrcChainId<Build, Target>,
            TargetDstChainId<Build, Target>,
            TargetSrcClientId<Build, Target>,
            TargetDstClientId<Build, Target>,
        ),
        TargetRelay<Build, Target>,
    >,
>;
