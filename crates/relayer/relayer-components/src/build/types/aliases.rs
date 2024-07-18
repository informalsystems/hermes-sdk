use alloc::collections::BTreeMap;

use cgp_core::error::{ErrorOf, HasErrorType};
use hermes_runtime_components::traits::mutex::MutexOf;
use hermes_runtime_components::traits::runtime::{HasRuntimeType, RuntimeOf};

use crate::birelay::traits::two_way::{HasTwoChainTypes, HasTwoWayRelayTypes};
use crate::build::traits::birelay::BiRelayOf;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::{ChainIdOf, ClientIdOf};
use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::relay_at::RelayTypeAt;
use crate::relay::traits::chains::HasRelayChains;

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

pub type ChainCacheAt<Build, const I: usize> =
    MutexOf<RuntimeOf<Build>, BTreeMap<ChainIdOf<ChainTypeAt<Build, I>>, ChainTypeAt<Build, I>>>;

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

pub type RelayCacheAt<Build, const SRC: usize, const DST: usize> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdOf<ChainTypeAt<Build, SRC>>,
            ChainIdOf<ChainTypeAt<Build, DST>>,
            ClientIdOf<ChainTypeAt<Build, SRC>, ChainTypeAt<Build, DST>>,
            ClientIdOf<ChainTypeAt<Build, DST>, ChainTypeAt<Build, SRC>>,
        ),
        RelayTypeAt<Build, SRC, DST>,
    >,
>;
