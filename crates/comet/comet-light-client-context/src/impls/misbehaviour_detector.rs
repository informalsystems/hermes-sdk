use hermes_comet_light_client_components::traits::{
    HasDivergenceType, HasLightBlockType, MisbehaviourDetector, MisbehaviourDetectorComponent,
};
use hermes_prelude::*;
use tendermint::crypto::default::Sha256;
use tendermint_light_client::builder::error::Error as LightClientBuilderError;
use tendermint_light_client::builder::LightClientBuilder;
use tendermint_light_client::components::clock::FixedClock;
use tendermint_light_client::components::io::ProdIo;
use tendermint_light_client::components::scheduler;
use tendermint_light_client::predicates::ProdPredicates;
use tendermint_light_client::store::memory::MemoryStore;
use tendermint_light_client_detector::{
    detect_divergence, Divergence, Error as LightClientDetectorError, Provider,
};
use tendermint_light_client_verifier::types::LightBlock;
use tendermint_light_client_verifier::ProdVerifier;
use tendermint_rpc::Error as RpcError;

use crate::traits::chain_id::HasChainId;
use crate::traits::current_time::HasCurrentTime;
use crate::traits::peer_id::HasPeerId;
use crate::traits::rpc_client::HasRpcClient;
use crate::traits::verifier_options::HasVerifierOptions;

pub struct DetectCometMisbehaviour;

#[cgp_provider(MisbehaviourDetectorComponent)]
impl<Client> MisbehaviourDetector<Client> for DetectCometMisbehaviour
where
    Client: HasLightBlockType<LightBlock = LightBlock>
        + HasDivergenceType<Divergence = Divergence>
        + HasPeerId
        + HasChainId
        + HasRpcClient
        + HasVerifierOptions
        + HasCurrentTime
        + CanRaiseAsyncError<RpcError>
        + CanRaiseAsyncError<LightClientBuilderError>
        + CanRaiseAsyncError<LightClientDetectorError>,
{
    async fn detect(
        client: &Client,
        target_block: &LightBlock,
        trusted_block: &LightBlock,
    ) -> Result<Option<Divergence>, Client::Error> {
        let peer_id = client.peer_id();
        let rpc_client = client.rpc_client();

        let now = client.current_time();

        let options = client.verifier_options();
        let light_store = Box::new(MemoryStore::new());

        let builder = LightClientBuilder::custom(
            *peer_id,
            *options,
            light_store,
            Box::new(ProdIo::new(*peer_id, rpc_client.clone(), None)),
            Box::new(FixedClock::new(now)),
            Box::<ProdVerifier>::default(),
            Box::new(scheduler::basic_bisecting_schedule),
            Box::new(ProdPredicates),
        );

        let instance = builder
            .trust_light_block(trusted_block.clone())
            .map_err(Client::raise_error)?
            .build();

        let mut provider = Provider::new(client.chain_id().clone(), instance, rpc_client.clone());

        tracing::warn!("provider: {provider:?}");
        tracing::warn!("clock_drift: {:?}", options.clock_drift);
        tracing::warn!("trusting_period: {:?}", options.trusting_period);

        detect_divergence::<Sha256>(
            None,
            &mut provider,
            vec![trusted_block.clone(), target_block.clone()],
            options.clock_drift,
            options.trusting_period,
        )
        .await
        .map_err(Client::raise_error)
    }
}
