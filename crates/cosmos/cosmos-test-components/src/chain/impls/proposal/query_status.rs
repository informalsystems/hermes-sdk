use core::fmt::Debug;

use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::HasGrpcAddress;
use hermes_test_components::chain::traits::{
    HasProposalIdType, HasProposalStatusType, ProposalStatusQuerier, ProposalStatusQuerierComponent,
};
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::cosmos::gov::v1::query_client::QueryClient;
use ibc_proto::cosmos::gov::v1::{Proposal, QueryProposalRequest};
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::chain::types::ProposalStatus;

pub struct QueryProposalStatusWithGrpc;

pub struct ProposalFailed<'a, Chain> {
    pub chain: &'a Chain,
    pub proposal: &'a Proposal,
}

#[cgp_provider(ProposalStatusQuerierComponent)]
impl<Chain> ProposalStatusQuerier<Chain> for QueryProposalStatusWithGrpc
where
    Chain: HasProposalIdType<ProposalId = u64>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<String>
        + for<'a> CanRaiseAsyncError<ProposalFailed<'a, Chain>>,
{
    async fn query_proposal_status(
        chain: &Chain,
        proposal_id: &u64,
    ) -> Result<ProposalStatus, Chain::Error> {
        let mut client = QueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?;

        let request = tonic::Request::new(QueryProposalRequest {
            proposal_id: *proposal_id,
        });

        let response = client
            .proposal(request)
            .await
            .map(|r| r.into_inner())
            .map_err(Chain::raise_error)?;

        let proposal = response
            .proposal
            .ok_or_else(|| Chain::raise_error(format!("proposal not found: {proposal_id}")))?;

        let proposal_status = match proposal.status {
            1 => ProposalStatus::DepositPeriod,
            2 => ProposalStatus::VotingPeriod,
            3 => ProposalStatus::Passed,
            4 => ProposalStatus::Rejected,
            5 => {
                return Err(Chain::raise_error(ProposalFailed {
                    chain,
                    proposal: &proposal,
                }))
            }
            _ => {
                return Err(Chain::raise_error(format!(
                    "unknown proposal status for proposal: {:?}",
                    proposal
                )));
            }
        };

        Ok(proposal_status)
    }
}

impl<Chain> Debug for ProposalFailed<'_, Chain> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProposalFailed")
            .field("id", &self.proposal.id)
            .field("status", &self.proposal.status)
            .field("final_tally_result", &self.proposal.final_tally_result)
            .field("submit_time", &self.proposal.submit_time)
            .field("deposit_end_time", &self.proposal.deposit_end_time)
            .field("total_deposit", &self.proposal.total_deposit)
            .field("voting_start_time", &self.proposal.voting_start_time)
            .field("voting_end_time", &self.proposal.voting_end_time)
            .field("metadata", &self.proposal.metadata)
            .field("title", &self.proposal.title)
            .field("summary", &self.proposal.summary)
            .field("proposer", &self.proposal.proposer)
            .finish()
    }
}
