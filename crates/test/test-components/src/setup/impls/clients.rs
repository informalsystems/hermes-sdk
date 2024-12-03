use core::marker::PhantomData;

use cgp::core::error::CanRaiseError;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{HasBoundedRelayTypeAt, RelayAt};
use hermes_relayer_components::multi::types::index::Twindex;
use hermes_relayer_components::relay::traits::chains::CanRaiseRelayChainErrors;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::setup::traits::clients::ClientSetup;
use crate::setup::traits::create_client_options_at::{
    HasCreateClientMessageOptionsAt, HasCreateClientPayloadOptionsAt,
};

pub struct SetupClientsWithRelay;

impl<Setup, A: Async, B: Async> ClientSetup<Setup, A, B> for SetupClientsWithRelay
where
    Setup: HasErrorType
        + HasBoundedRelayTypeAt<A, B>
        + HasCreateClientPayloadOptionsAt<A, B>
        + HasCreateClientPayloadOptionsAt<B, A>
        + HasCreateClientMessageOptionsAt<A, B>
        + HasCreateClientMessageOptionsAt<B, A>
        + CanRaiseError<<RelayAt<Setup, A, B> as HasErrorType>::Error>,
    ChainAt<Setup, A>: HasIbcChainTypes<ChainAt<Setup, B>>
        + HasCreateClientPayloadOptionsType<ChainAt<Setup, B>>
        + HasCreateClientMessageOptionsType<ChainAt<Setup, B>>
        + HasErrorType,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>>
        + HasCreateClientPayloadOptionsType<ChainAt<Setup, A>>
        + HasCreateClientMessageOptionsType<ChainAt<Setup, A>>
        + HasErrorType,
    RelayAt<Setup, A, B>: CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanRaiseRelayChainErrors,
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
        let client_id_a = <RelayAt<Setup, A, B>>::create_client(
            SourceTarget,
            chain_a,
            chain_b,
            setup.create_client_payload_options(PhantomData::<(B, A)>),
            setup.create_client_message_options(PhantomData::<(A, B)>),
        )
        .await
        .map_err(Setup::raise_error)?;

        let client_id_b = <RelayAt<Setup, A, B>>::create_client(
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
