use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::types::WithDelegatedType;
use hermes_chain_components::traits::{
    EvidenceTypeProviderComponent, HasHeightType, HeightTypeProviderComponent,
};
use hermes_comet_light_client_components::impls::DoVerifyForward;
use hermes_comet_light_client_components::traits::{
    CanBuildLightBlocksForUpdateClient, CanComputeNextVerificationHeight, CanDetectMisbehaviour,
    CanFetchLightBlock, CanFetchLightBlockWithStatus, CanQueryLightBlock,
    CanTraceVerificationHeight, CanUpdateVerificationStatus, CanValidateLightBlock,
    CanVerifyTargetHeight, CanVerifyUpdateHeader, DivergenceTypeProviderComponent,
    GetHighestTrustedOrVerifiedBefore, HasLightBlockHeight, HasLightBlockType, HasVerdictType,
    HasVerificationStatusType, IsWithinTrustingPeriod, LightBlockFetcherComponent,
    LightBlockHeightGetterComponent, LightBlockQuerierComponent, LightBlockTypeComponent,
    LightBlockValidatorComponent, LightBlockWithStatusFetcherComponent,
    LightBlocksForUpdateClientBuilderComponent, MisbehaviourDetectorComponent,
    NextVerificationHeightComputerComponent, TargetHeightVerifierComponent, TrustedStatus,
    UpdateHeaderVerifierComponent, VerdictTypeComponent, VerificationHeightTracerComponent,
    VerificationStatusTypeComponent, VerificationStatusUpdaterComponent, VerifiedStatus,
    VerifyForward,
};
use hermes_comet_light_client_components::types::{Verdict, VerificationStatus};
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use tendermint::block::Height;
use tendermint::Time;
use tendermint_light_client_verifier::options::Options;
use tendermint_light_client_verifier::types::{LightBlock, PeerId};
use tendermint_light_client_verifier::ProdVerifier;
use tendermint_rpc::HttpClient;

use crate::contexts::error::HandleLightClientError;
use crate::impls::bisect_height::BisectHeight;
use crate::impls::fetch_light_block::FetchTendermintLightBlock;
use crate::impls::fetch_light_block_with_status::FetchTendermintLightBlockWithStatus;
use crate::impls::misbehaviour_detector::DetectCometMisbehaviour;
use crate::impls::query_light_block::highest_trusted_or_before::QueryHighestTrustedOrVerifiedBefore;
use crate::impls::trace_verification::TraceTendermintVerification;
use crate::impls::types::all::CometLightClientTypes;
use crate::impls::types::light_block::UseTendermintLightBlock;
use crate::impls::update_client::BuildTendermintUpdateClientBlocks;
use crate::impls::update_verification_status::DoUpdateVerifactionStatus;
use crate::impls::validate_light_block::ValidateTendermintLightBlock;
use crate::impls::verify_update_header::VerifyUpdateHeaderWithProdVerifier;
use crate::traits::chain_id::ChainIdGetterComponent;
use crate::traits::current_time::{CurrentTimeGetterComponent, HasCurrentTime};
use crate::traits::light_block_store::{
    HasLightBlockStore, LightBlockStore, LightBlockStoreGetterComponent,
};
use crate::traits::peer_id::PeerIdGetterComponent;
use crate::traits::rpc_client::RpcClientGetterComponent;
use crate::traits::verification_trace::{VerificationTrace, VerificationTraceGetterComponent};
use crate::traits::verifier::VerifierComponent;
use crate::traits::verifier_options::{HasVerifierOptions, VerifierOptionsGetterComponent};

#[cgp_context(CometLightClientComponents)]
#[derive(HasField)]
pub struct CometLightClient {
    pub chain_id: String,
    pub current_time: Time,
    pub peer_id: PeerId,
    pub rpc_client: HttpClient,
    pub verifier_options: Options,
    pub light_block_store: LightBlockStore,
    pub verification_trace: VerificationTrace,
    pub verifier: ProdVerifier,
}

delegate_components! {
    CometLightClientComponents {
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleLightClientError>,
        [
            LightBlockTypeComponent,
            LightBlockHeightGetterComponent,
        ]:
            UseTendermintLightBlock,
        [
            HeightTypeProviderComponent,
            VerificationStatusTypeComponent,
            VerdictTypeComponent,
            DivergenceTypeProviderComponent,
            EvidenceTypeProviderComponent,
        ]:
            WithDelegatedType<CometLightClientTypes>,
        [
            CurrentTimeGetterComponent,
            PeerIdGetterComponent,
            RpcClientGetterComponent,
            ChainIdGetterComponent,
            VerifierOptionsGetterComponent,
            LightBlockStoreGetterComponent,
            VerificationTraceGetterComponent,
            VerifierComponent,
        ]:
            UseFields,
        LightBlockQuerierComponent:
            QueryHighestTrustedOrVerifiedBefore,
        NextVerificationHeightComputerComponent:
            BisectHeight,
        LightBlockFetcherComponent:
            FetchTendermintLightBlock,
        LightBlockWithStatusFetcherComponent:
            FetchTendermintLightBlockWithStatus,
        VerificationHeightTracerComponent:
            TraceTendermintVerification,
        VerificationStatusUpdaterComponent:
            DoUpdateVerifactionStatus,
        LightBlockValidatorComponent:
            ValidateTendermintLightBlock,
        MisbehaviourDetectorComponent:
            DetectCometMisbehaviour,
        UpdateHeaderVerifierComponent:
            VerifyUpdateHeaderWithProdVerifier,
        TargetHeightVerifierComponent:
            DoVerifyForward,
        LightBlocksForUpdateClientBuilderComponent:
            BuildTendermintUpdateClientBlocks,
    }
}

impl CometLightClient {
    pub fn new(
        chain_id: String,
        current_time: Time,
        peer_id: PeerId,
        rpc_client: HttpClient,
        verifier_options: Options,
    ) -> Self {
        Self {
            chain_id,
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

pub trait CanUseCometLightClient:
    Async
    + HasHeightType<Height = Height>
    + HasLightBlockType<LightBlock = LightBlock>
    + HasVerificationStatusType<VerificationStatus = VerificationStatus>
    + HasVerdictType<Verdict = Verdict>
    + HasLightBlockHeight
    + HasLightBlockStore
    + HasVerifierOptions
    + CanDetectMisbehaviour
    + HasCurrentTime
    + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
    + CanValidateLightBlock<IsWithinTrustingPeriod>
    + CanUpdateVerificationStatus<VerifiedStatus>
    + CanUpdateVerificationStatus<TrustedStatus>
    + CanComputeNextVerificationHeight
    + CanFetchLightBlock
    + CanFetchLightBlockWithStatus
    + CanTraceVerificationHeight
    + CanVerifyUpdateHeader
    + CanVerifyTargetHeight<VerifyForward>
    + CanBuildLightBlocksForUpdateClient
{
}

impl CanUseCometLightClient for CometLightClient {}
