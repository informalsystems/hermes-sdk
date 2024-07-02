use cgp_core::prelude::{async_trait, DelegateComponent, HasComponents, HasErrorType};
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::build::traits::target::relay::RelayBuildTarget;
use hermes_relayer_components::build::types::aliases::{
    TargetDstChain, TargetDstClientId, TargetRelay, TargetSrcChain, TargetSrcClientId,
};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::channel::HasMessageBatchSenderTypes;

pub struct RelayWithBatchBuilderComponent;

#[async_trait]
pub trait RelayWithBatchBuilder<Build, Target>
where
    Build: HasBiRelayType + HasErrorType,
    Target: RelayBuildTarget<Build>,
    Target::TargetRelay: HasMessageBatchSenderTypes,
{
    async fn build_relay_with_batch(
        build: &Build,
        src_client_id: &TargetSrcClientId<Build, Target>,
        dst_client_id: &TargetDstClientId<Build, Target>,
        src_chain: TargetSrcChain<Build, Target>,
        dst_chain: TargetDstChain<Build, Target>,
        src_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::SrcMessageBatchSender,
        dst_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::DstMessageBatchSender,
    ) -> Result<TargetRelay<Build, Target>, Build::Error>;
}

impl<Build, Target, Component> RelayWithBatchBuilder<Build, Target> for Component
where
    Build: HasBiRelayType + HasErrorType,
    Target: RelayBuildTarget<Build>,
    Target::TargetRelay: HasMessageBatchSenderTypes,
    Component: DelegateComponent<RelayWithBatchBuilderComponent>,
    Component::Delegate: RelayWithBatchBuilder<Build, Target>,
{
    async fn build_relay_with_batch(
        build: &Build,
        src_client_id: &TargetSrcClientId<Build, Target>,
        dst_client_id: &TargetDstClientId<Build, Target>,
        src_chain: TargetSrcChain<Build, Target>,
        dst_chain: TargetDstChain<Build, Target>,
        src_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::SrcMessageBatchSender,
        dst_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::DstMessageBatchSender,
    ) -> Result<TargetRelay<Build, Target>, Build::Error> {
        Component::Delegate::build_relay_with_batch(
            build,
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )
        .await
    }
}

#[async_trait]
pub trait CanBuildRelayWithBatch<Target>: HasBiRelayType + HasRuntime + HasErrorType
where
    Target: RelayBuildTarget<Self>,
    Target::TargetRelay: HasMessageBatchSenderTypes,
{
    async fn build_relay_with_batch(
        &self,
        target: Target,
        src_client_id: &TargetSrcClientId<Self, Target>,
        dst_client_id: &TargetDstClientId<Self, Target>,
        src_chain: TargetSrcChain<Self, Target>,
        dst_chain: TargetDstChain<Self, Target>,
        src_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::SrcMessageBatchSender,
        dst_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::DstMessageBatchSender,
    ) -> Result<TargetRelay<Self, Target>, Self::Error>;
}

impl<Build, Target> CanBuildRelayWithBatch<Target> for Build
where
    Build: HasBiRelayType + HasRuntime + HasErrorType + HasComponents,
    Target: RelayBuildTarget<Build>,
    Target::TargetRelay: HasMessageBatchSenderTypes,
    Build::Components: RelayWithBatchBuilder<Build, Target>,
{
    async fn build_relay_with_batch(
        &self,
        _target: Target,
        src_client_id: &TargetSrcClientId<Self, Target>,
        dst_client_id: &TargetDstClientId<Self, Target>,
        src_chain: TargetSrcChain<Self, Target>,
        dst_chain: TargetDstChain<Self, Target>,
        src_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::SrcMessageBatchSender,
        dst_batch_sender: <Target::TargetRelay as HasMessageBatchSenderTypes>::DstMessageBatchSender,
    ) -> Result<TargetRelay<Self, Target>, Self::Error> {
        Build::Components::build_relay_with_batch(
            self,
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )
        .await
    }
}
