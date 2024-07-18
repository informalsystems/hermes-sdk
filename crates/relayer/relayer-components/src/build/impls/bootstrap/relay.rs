use cgp_core::prelude::{async_trait, CanRaiseError, HasErrorType};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::builders::chain_builder::CanBuildChain;
use crate::build::traits::builders::relay_builder::CanBuildRelay;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{
    RelayError, TargetDstChain, TargetDstChainId, TargetRelay, TargetSrcChain, TargetSrcChainId,
};
use crate::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::client_creator::CanCreateClient;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};

#[async_trait]
pub trait CanBootstrapRelay<Target>: HasBiRelayType + HasErrorType
where
    Target: RelayBuildTarget<Self>,
    TargetSrcChain<Self, Target>: HasCreateClientPayloadOptionsType<TargetDstChain<Self, Target>>
        + HasCreateClientMessageOptionsType<TargetDstChain<Self, Target>>,
    TargetDstChain<Self, Target>: HasCreateClientPayloadOptionsType<TargetSrcChain<Self, Target>>
        + HasCreateClientMessageOptionsType<TargetSrcChain<Self, Target>>,
{
    async fn bootstrap_relay(
        &self,
        target: Target,
        src_chain_id: &TargetSrcChainId<Self, Target>,
        dst_chain_id: &TargetDstChainId<Self, Target>,
        src_payload_options: &<TargetSrcChain<Self, Target> as HasCreateClientPayloadOptionsType<
            TargetDstChain<Self, Target>,
        >>::CreateClientPayloadOptions,
        dst_payload_options: &<TargetDstChain<Self, Target> as HasCreateClientPayloadOptionsType<
            TargetSrcChain<Self, Target>,
        >>::CreateClientPayloadOptions,
        src_message_options: &<TargetSrcChain<Self, Target> as HasCreateClientMessageOptionsType<
            TargetDstChain<Self, Target>,
        >>::CreateClientMessageOptions,
        dst_message_options: &<TargetDstChain<Self, Target> as HasCreateClientMessageOptionsType<
            TargetSrcChain<Self, Target>,
        >>::CreateClientMessageOptions,
    ) -> Result<TargetRelay<Self, Target>, Self::Error>;
}

impl<Build, Target, BiRelay, Relay, SrcChain, DstChain> CanBootstrapRelay<Target> for Build
where
    Build: HasBiRelayType<BiRelay = BiRelay>
        + CanBuildRelay<Target>
        + CanBuildChain<Target::SrcChainTarget>
        + CanBuildChain<Target::DstChainTarget>
        + CanRaiseError<BiRelay::Error>,
    BiRelay: HasTwoWayRelay + CanRaiseError<Relay::Error>,
    Target: RelayBuildTarget<Self, TargetRelay = Relay>,
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Error = RelayError<Build>>
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanRaiseRelayChainErrors,
    SrcChain: HasCreateClientPayloadOptionsType<DstChain>
        + HasCreateClientMessageOptionsType<DstChain>
        + HasIbcChainTypes<DstChain>
        + HasErrorType,
    DstChain: HasCreateClientPayloadOptionsType<SrcChain>
        + HasCreateClientMessageOptionsType<SrcChain>
        + HasIbcChainTypes<SrcChain>
        + HasErrorType,
{
    async fn bootstrap_relay(
        &self,
        target: Target,
        src_chain_id: &SrcChain::ChainId,
        dst_chain_id: &DstChain::ChainId,
        src_payload_options: &SrcChain::CreateClientPayloadOptions,
        dst_payload_options: &DstChain::CreateClientPayloadOptions,
        src_message_options: &SrcChain::CreateClientMessageOptions,
        dst_message_options: &DstChain::CreateClientMessageOptions,
    ) -> Result<Relay, Self::Error> {
        let src_chain = self
            .build_chain(Target::SrcChainTarget::default(), src_chain_id)
            .await?;

        let dst_chain = self
            .build_chain(Target::DstChainTarget::default(), dst_chain_id)
            .await?;

        let src_client_id = Relay::create_client(
            SourceTarget,
            &src_chain,
            &dst_chain,
            dst_payload_options,
            src_message_options,
        )
        .await
        .map_err(Build::BiRelay::raise_error)
        .map_err(Build::raise_error)?;

        let dst_client_id = Relay::create_client(
            DestinationTarget,
            &dst_chain,
            &src_chain,
            src_payload_options,
            dst_message_options,
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
