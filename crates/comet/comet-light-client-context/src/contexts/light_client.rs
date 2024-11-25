use cgp::core::component::UseContext;
use cgp::core::types::impls::WithDelegatedType;
use cgp::prelude::*;
use hermes_chain_components::traits::types::height::{HasHeightType, HeightTypeComponent};
use hermes_comet_light_client_components::traits::light_block::height::LightBlockHeightGetterComponent;
use hermes_comet_light_client_components::traits::types::light_block::LightBlockTypeComponent;
use hermes_comet_light_client_components::traits::types::status::VerificationStatusTypeComponent;
use hermes_comet_light_client_components::traits::types::verdict::VerdictTypeComponent;
use tendermint::Time;
use tendermint_light_client_verifier::options::Options;
use tendermint_light_client_verifier::types::PeerId;
use tendermint_light_client_verifier::ProdVerifier;
use tendermint_rpc::HttpClient;

use crate::impls::types::all::CometLightClientTypes;
use crate::impls::types::light_block::UseTendermintLightBlock;
use crate::traits::current_time::CurrentTimeGetterComponent;
use crate::traits::light_block_store::{LightBlockStore, LightBlockStoreGetterComponent};
use crate::traits::peer_id::PeerIdGetterComponent;
use crate::traits::rpc_client::RpcClientGetterComponent;
use crate::traits::verification_trace::{VerificationTrace, VerificationTraceGetterComponent};
use crate::traits::verifier::VerifierComponent;

#[derive(HasField)]
pub struct CometLightClient {
    pub current_time: Time,
    pub peer_id: PeerId,
    pub rpc_client: HttpClient,
    pub verifier_options: Options,
    pub light_block_store: LightBlockStore,
    pub verification_trace: VerificationTrace,
    pub verifier: ProdVerifier,
}

pub struct CometLightClientComponents;

impl HasComponents for CometLightClient {
    type Components = CometLightClientComponents;
}

delegate_components! {
    CometLightClientComponents {
        [
            LightBlockTypeComponent,
            LightBlockHeightGetterComponent,
        ]:
            UseTendermintLightBlock,
        [
            HeightTypeComponent,
            VerificationStatusTypeComponent,
            VerdictTypeComponent,
        ]:
            WithDelegatedType<CometLightClientTypes>,
        [
            CurrentTimeGetterComponent,
            PeerIdGetterComponent,
            RpcClientGetterComponent,
            LightBlockStoreGetterComponent,
            VerificationTraceGetterComponent,
            VerifierComponent
        ]:
            UseContext,
    }
}

impl CometLightClient {
    pub fn new(
        current_time: Time,
        peer_id: PeerId,
        rpc_client: HttpClient,
        verifier_options: Options,
    ) -> Self {
        Self {
            current_time,
            peer_id,
            rpc_client,
            verifier_options,
            light_block_store: Default::default(),
            verification_trace: Default::default(),
            verifier: Default::default(),
        }
    }
}

pub trait CanUseCometLightClient: HasHeightType {}

impl CanUseCometLightClient for CometLightClient {}
