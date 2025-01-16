use core::marker::PhantomData;

use cgp::core::error::CanRaiseAsyncError;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::relay::traits::chains::{HasRelayChainTypes, HasRelayClientIds};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};

use crate::setup::traits::clients::ClientSetup;
use crate::setup::traits::create_client_options_at::{
    HasCreateClientMessageOptionsAt, HasCreateClientPayloadOptionsAt,
};

pub struct SetupClientsWithRelay;

impl<Setup, A: Async, B: Async, Relay, SrcChain, DstChain> ClientSetup<Setup, A, B>
    for SetupClientsWithRelay
where
    Setup: HasRelayTypeAt<A, B, Relay = Relay>
        + HasChainTypeAt<A, Chain = SrcChain>
        + HasChainTypeAt<B, Chain = DstChain>
        + HasCreateClientPayloadOptionsAt<A, B>
        + HasCreateClientPayloadOptionsAt<B, A>
        + HasCreateClientMessageOptionsAt<A, B>
        + HasCreateClientMessageOptionsAt<B, A>
        + CanRaiseAsyncError<Relay::Error>,
    SrcChain: HasClientIdType<DstChain>
        + HasCreateClientPayloadOptionsType<DstChain>
        + HasCreateClientMessageOptionsType<DstChain>
        + HasAsyncErrorType,
    DstChain: HasClientIdType<SrcChain>
        + HasCreateClientPayloadOptionsType<SrcChain>
        + HasCreateClientMessageOptionsType<SrcChain>
        + HasAsyncErrorType,
    Relay: HasRelayChainTypes<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
{
    async fn setup_clients(
        setup: &Setup,
        chain_a: &ChainAt<Setup, A>,
        chain_b: &ChainAt<Setup, B>,
    ) -> Result<
        (
            ClientIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
            ClientIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
        ),
        Setup::Error,
    > {
        let client_id_a = Relay::create_client(
            SourceTarget,
            chain_a,
            chain_b,
            setup.create_client_payload_options(PhantomData::<(B, A)>),
            setup.create_client_message_options(PhantomData::<(A, B)>),
        )
        .await
        .map_err(Setup::raise_error)?;

        let client_id_b = Relay::create_client(
            DestinationTarget,
            chain_b,
            chain_a,
            setup.create_client_payload_options(PhantomData::<(A, B)>),
            setup.create_client_message_options(PhantomData::<(B, A)>),
        )
        .await
        .map_err(Setup::raise_error)?;

        Ok((client_id_a, client_id_b))
    }
}
