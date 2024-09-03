use cgp::core::error::CanRaiseError;
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_test_components::chain::traits::proposal::query_status::ProposalStatusQuerier;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use ibc_proto::cosmos::gov::v1::query_client::QueryClient;
use ibc_proto::cosmos::gov::v1::QueryProposalRequest;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::chain::types::proposal_status::ProposalStatus;

pub struct QueryProposalStatusWithGrpc;

impl<Chain> ProposalStatusQuerier<Chain> for QueryProposalStatusWithGrpc
where
    Chain: HasProposalIdType<ProposalId = u64>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasGrpcAddress
        + CanRaiseError<Status>
        + CanRaiseError<TransportError>
        + CanRaiseError<String>,
{
    async fn query_proposal_status(
        chain: &Chain,
        proposal_id: &u64,
    ) -> Result<ProposalStatus, Chain::Error> {
        let grpc_address = chain.grpc_address();

        let mut client = QueryClient::connect(grpc_address.clone())
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
            5 => ProposalStatus::Failed,
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
