use cgp_core::{async_trait, CanRaiseError, HasErrorType};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::components::chain_builder::CanBuildChain;
use crate::build::traits::components::relay_builder::CanBuildRelay;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{
    RelayError, TargetDstChain, TargetDstChainId, TargetRelay, TargetSrcChain, TargetSrcChainId,
};
use crate::chain::traits::types::create_client::HasCreateClientOptions;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::client_creator::CanCreateClient;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};

#[async_trait]
pub trait CanBootstrapRelay<Target>: HasBiRelayType + HasErrorType
where
    Target: RelayBuildTarget<Self>,
    TargetSrcChain<Self, Target>: HasCreateClientOptions<TargetDstChain<Self, Target>>,
    TargetDstChain<Self, Target>: HasCreateClientOptions<TargetSrcChain<Self, Target>>,
{
    async fn bootstrap_relay(
        &self,
        target: Target,
        src_chain_id: &TargetSrcChainId<Self, Target>,
        dst_chain_id: &TargetDstChainId<Self, Target>,
        src_options: &<TargetSrcChain<Self, Target> as HasCreateClientOptions<
            TargetDstChain<Self, Target>,
        >>::CreateClientPayloadOptions,
        dst_options: &<TargetDstChain<Self, Target> as HasCreateClientOptions<
            TargetSrcChain<Self, Target>,
        >>::CreateClientPayloadOptions,
    ) -> Result<TargetRelay<Self, Target>, Self::Error>;
}

#[async_trait]
impl<Build, Target, Relay, SrcChain, DstChain> CanBootstrapRelay<Target> for Build
where
    Build: CanBuildRelay<Target>
        + CanBuildChain<Target::SrcChainTarget>
        + CanBuildChain<Target::DstChainTarget>,
    Build::BiRelay: HasTwoWayRelay,
    Target: RelayBuildTarget<Self, TargetRelay = Relay>,
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Error = RelayError<Build>>
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>,
    SrcChain: HasCreateClientOptions<DstChain> + HasIbcChainTypes<DstChain> + HasErrorType,
    DstChain: HasCreateClientOptions<SrcChain> + HasIbcChainTypes<SrcChain> + HasErrorType,
{
    async fn bootstrap_relay(
        &self,
        target: Target,
        src_chain_id: &SrcChain::ChainId,
        dst_chain_id: &DstChain::ChainId,
        src_payload_options: &SrcChain::CreateClientPayloadOptions,
        dst_payload_options: &DstChain::CreateClientPayloadOptions,
    ) -> Result<Relay, Self::Error> {
        let src_chain = self
            .build_chain(Target::SrcChainTarget::default(), src_chain_id)
            .await?;

        let dst_chain = self
            .build_chain(Target::DstChainTarget::default(), dst_chain_id)
            .await?;

        let src_client_id =
            Relay::create_client(SourceTarget, &src_chain, &dst_chain, dst_payload_options)
                .await
                .map_err(Build::BiRelay::raise_error)
                .map_err(Build::raise_error)?;

        let dst_client_id = Relay::create_client(
            DestinationTarget,
            &dst_chain,
            &src_chain,
            src_payload_options,
        )
        .await
        .map_err(Build::BiRelay::raise_error)
        .map_err(Build::raise_error)?;

        let relay = self
            .build_relay(
                target,
                src_chain_id,
                dst_chain_id,
                &src_client_id,
                &dst_client_id,
            )
            .await?;

        Ok(relay)
    }
}
