use cgp::core::component::{UseContext, UseDelegate};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::types::WithDelegatedType;
use cgp::prelude::*;
use hermes_chain_components::traits::types::height::{HasHeightType, HeightTypeComponent};
use hermes_comet_light_client_components::impls::verify_target_height::verify_forward::DoVerifyForward;
use hermes_comet_light_client_components::traits::compute_verification_height::{
    CanComputeNextVerificationHeight, NextVerificationHeightComputerComponent,
};
use hermes_comet_light_client_components::traits::fetch_light_block::{
    CanFetchLightBlock, CanFetchLightBlockWithStatus, LightBlockFetcherComponent,
    LightBlockWithStatusFetcherComponent,
};
use hermes_comet_light_client_components::traits::light_block::height::{
    HasLightBlockHeight, LightBlockHeightGetterComponent,
};
use hermes_comet_light_client_components::traits::query_light_block::{
    CanQueryLightBlock, GetHighestTrustedOrVerifiedBefore, LightBlockQuerierComponent,
};
use hermes_comet_light_client_components::traits::trace_verification_height::{
    CanTraceVerificationHeight, VerificationHeightTracerComponent,
};
use hermes_comet_light_client_components::traits::types::light_block::{
    HasLightBlockType, LightBlockTypeComponent,
};
use hermes_comet_light_client_components::traits::types::status::{
    HasVerificationStatusType, VerificationStatusTypeComponent,
};
use hermes_comet_light_client_components::traits::types::verdict::{
    HasVerdictType, VerdictTypeComponent,
};
use hermes_comet_light_client_components::traits::update_client::{
    CanBuildLightBlocksForUpdateClient, LightBlocksForUpdateClientBuilderComponent,
};
use hermes_comet_light_client_components::traits::update_verification_status::{
    CanUpdateVerificationStatus, TrustedStatus, VerificationStatusUpdaterComponent, VerifiedStatus,
};
use hermes_comet_light_client_components::traits::validate_light_block::{
    CanValidateLightBlock, IsWithinTrustingPeriod, LightBlockValidatorComponent,
};
use hermes_comet_light_client_components::traits::verify_target_height::{
    CanVerifyTargetHeight, TargetHeightVerifierComponent, VerifyForward,
};
use hermes_comet_light_client_components::traits::verify_update_header::{
    CanVerifyUpdateHeader, UpdateHeaderVerifierComponent,
};
use hermes_comet_light_client_components::types::status::VerificationStatus;
use hermes_comet_light_client_components::types::verdict::Verdict;
use hermes_error::impls::ProvideHermesError;
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
use crate::impls::query_light_block::highest_trusted_or_before::QueryHighestTrustedOrVerifiedBefore;
use crate::impls::trace_verification::TraceTendermintVerification;
use crate::impls::types::all::CometLightClientTypes;
use crate::impls::types::light_block::UseTendermintLightBlock;
use crate::impls::update_client::BuildTendermintUpdateClientBlocks;
use crate::impls::update_verification_status::DoUpdateVerifactionStatus;
use crate::impls::validate_light_block::ValidateTendermintLightBlock;
use crate::impls::verify_update_header::VerifyUpdateHeaderWithProdVerifier;
use crate::traits::current_time::{CurrentTimeGetterComponent, HasCurrentTime};
use crate::traits::light_block_store::{
    HasLightBlockStore, LightBlockStore, LightBlockStoreGetterComponent,
};
use crate::traits::peer_id::PeerIdGetterComponent;
use crate::traits::rpc_client::RpcClientGetterComponent;
use crate::traits::verification_trace::{VerificationTrace, VerificationTraceGetterComponent};
use crate::traits::verifier::VerifierComponent;
use crate::traits::verifier_options::{HasVerifierOptions, VerifierOptionsGetterComponent};

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
        ErrorTypeProviderComponent:
            ProvideHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleLightClientError>,
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
            VerifierOptionsGetterComponent,
            LightBlockStoreGetterComponent,
            VerificationTraceGetterComponent,
            VerifierComponent,
        ]:
            UseContext,
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

pub trait CanUseCometLightClient:
    Async
    + HasHeightType<Height = Height>
    + HasLightBlockType<LightBlock = LightBlock>
    + HasVerificationStatusType<VerificationStatus = VerificationStatus>
    + HasVerdictType<Verdict = Verdict>
    + HasLightBlockHeight
    + HasLightBlockStore
    + HasVerifierOptions
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
