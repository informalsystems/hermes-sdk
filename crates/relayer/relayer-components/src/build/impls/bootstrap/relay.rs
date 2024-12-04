use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::Async;
use cgp::prelude::{async_trait, CanRaiseError, HasErrorType};

use crate::build::traits::builders::chain_builder::CanBuildChain;
use crate::build::traits::builders::relay_builder::CanBuildRelay;
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::HasRelayTypeAt;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayClientIds};
use crate::relay::traits::client_creator::CanCreateClient;
use crate::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};

#[async_trait]
pub trait CanBootstrapRelay<Src, Dst>:
    HasRelayTypeAt<Src, Dst>
    + HasChainTypeAt<
        Src,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, Dst>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, Dst>>,
    > + HasChainTypeAt<
        Dst,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, Src>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, Src>>,
    > + HasErrorType
{
    async fn bootstrap_relay(
        &self,
        _tag: PhantomData<(Src, Dst)>,
        src_chain_id: &ChainIdAt<Self, Src>,
        dst_chain_id: &ChainIdAt<Self, Dst>,
        src_payload_options: &CreateClientPayloadOptionsOf<ChainAt<Self, Src>, ChainAt<Self, Dst>>,
        dst_payload_options: &CreateClientPayloadOptionsOf<ChainAt<Self, Dst>, ChainAt<Self, Src>>,
        src_message_options: &CreateClientMessageOptionsOf<ChainAt<Self, Src>, ChainAt<Self, Dst>>,
        dst_message_options: &CreateClientMessageOptionsOf<ChainAt<Self, Dst>, ChainAt<Self, Src>>,
    ) -> Result<Self::Relay, Self::Error>;
}

impl<Build, SrcChain, DstChain, Src: Async, Dst: Async> CanBootstrapRelay<Src, Dst> for Build
where
    Build: CanBuildRelay<Src, Dst, Relay: HasRelayClientIds>
        + CanBuildChain<Src, Chain = SrcChain>
        + CanBuildChain<Dst, Chain = DstChain>
        + CanRaiseError<ErrorOf<Build::Relay>>,
    Build::Relay: HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
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
        tag: PhantomData<(Src, Dst)>,
        src_chain_id: &SrcChain::ChainId,
        dst_chain_id: &DstChain::ChainId,
        src_payload_options: &SrcChain::CreateClientPayloadOptions,
        dst_payload_options: &DstChain::CreateClientPayloadOptions,
        src_message_options: &SrcChain::CreateClientMessageOptions,
        dst_message_options: &DstChain::CreateClientMessageOptions,
    ) -> Result<Build::Relay, Self::Error> {
        let src_chain = self.build_chain(PhantomData::<Src>, src_chain_id).await?;

        let dst_chain = self.build_chain(PhantomData::<Dst>, dst_chain_id).await?;

        let src_client_id = Build::Relay::create_client(
            SourceTarget,
            &src_chain,
            &dst_chain,
            dst_payload_options,
            src_message_options,
        )
        .await
        .map_err(Build::raise_error)?;

        let dst_client_id = Build::Relay::create_client(
            DestinationTarget,
            &dst_chain,
            &src_chain,
            src_payload_options,
            dst_message_options,
        )
        .await
        .map_err(Build::raise_error)?;

        let relay = self
            .build_relay(
                tag,
                src_chain_id,
                dst_chain_id,
                &src_client_id,
                &dst_client_id,
            )
            .await?;

        Ok(relay)
    }
}
