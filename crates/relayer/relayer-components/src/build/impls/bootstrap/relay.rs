use cgp_core::error::ErrorOf;
use cgp_core::prelude::{async_trait, CanRaiseError, HasErrorType};

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
use crate::multi::types::index::{Index, Twindex};
use crate::relay::traits::chains::CanRaiseRelayChainErrors;
use crate::relay::traits::client_creator::CanCreateClient;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};

#[async_trait]
pub trait CanBootstrapRelay<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST>
    + HasChainTypeAt<
        SRC,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, DST>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, DST>>,
    > + HasChainTypeAt<
        DST,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, SRC>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, SRC>>,
    > + HasErrorType
{
    async fn bootstrap_relay(
        &self,
        index: Twindex<SRC, DST>,
        src_chain_id: &ChainIdAt<Self, SRC>,
        dst_chain_id: &ChainIdAt<Self, DST>,
        src_payload_options: &CreateClientPayloadOptionsOf<ChainAt<Self, SRC>, ChainAt<Self, DST>>,
        dst_payload_options: &CreateClientPayloadOptionsOf<ChainAt<Self, DST>, ChainAt<Self, SRC>>,
        src_message_options: &CreateClientMessageOptionsOf<ChainAt<Self, SRC>, ChainAt<Self, DST>>,
        dst_message_options: &CreateClientMessageOptionsOf<ChainAt<Self, DST>, ChainAt<Self, SRC>>,
    ) -> Result<Self::Relay, Self::Error>;
}

impl<Build, SrcChain, DstChain, const SRC: usize, const DST: usize> CanBootstrapRelay<SRC, DST>
    for Build
where
    Build: CanBuildRelay<SRC, DST>
        + CanBuildChain<SRC, Chain = SrcChain>
        + CanBuildChain<DST, Chain = DstChain>
        + CanRaiseError<ErrorOf<Build::Relay>>,
    Build::Relay: CanCreateClient<SourceTarget>
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
        index: Twindex<SRC, DST>,
        src_chain_id: &SrcChain::ChainId,
        dst_chain_id: &DstChain::ChainId,
        src_payload_options: &SrcChain::CreateClientPayloadOptions,
        dst_payload_options: &DstChain::CreateClientPayloadOptions,
        src_message_options: &SrcChain::CreateClientMessageOptions,
        dst_message_options: &DstChain::CreateClientMessageOptions,
    ) -> Result<Build::Relay, Self::Error> {
        let src_chain = self.build_chain(Index::<SRC>, src_chain_id).await?;

        let dst_chain = self.build_chain(Index::<DST>, dst_chain_id).await?;

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
                index,
                src_chain_id,
                dst_chain_id,
                &src_client_id,
                &dst_client_id,
            )
            .await?;

        Ok(relay)
    }
}
