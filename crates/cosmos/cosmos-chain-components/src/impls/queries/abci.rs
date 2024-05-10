use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::core::ics23_commitment::error::Error as Ics23Error;
use ibc_relayer_types::core::ics23_commitment::merkle::convert_tm_to_ics_merkle_proof;
use ibc_relayer_types::proofs::ProofError;
use tendermint::block::Height as TendermintHeight;
use tendermint_rpc::endpoint::abci_query::AbciQuery;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::abci_query::AbciQuerier;
use crate::traits::rpc_client::HasRpcClient;

pub struct QueryAbci;

#[derive(Debug)]
pub struct AbciQueryError {
    pub response: AbciQuery,
}

impl<Chain> AbciQuerier<Chain> for QueryAbci
where
    Chain: HasRpcClient
        + HasHeightType
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + CanRaiseError<RpcError>
        + CanRaiseError<AbciQueryError>
        + CanRaiseError<Ics23Error>
        + CanRaiseError<ProofError>
        + CanRaiseError<&'static str>,
    Chain::Height: Clone + Into<TendermintHeight>,
{
    async fn query_abci(
        chain: &Chain,
        path: &str,
        data: &[u8],
        height: &Chain::Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let response = chain
            .rpc_client()
            .abci_query(
                Some(path.to_owned()),
                data,
                Some(height.clone().into()),
                false,
            )
            .await
            .map_err(Chain::raise_error)?;

        if !response.code.is_ok() {
            return Err(Chain::raise_error(AbciQueryError { response }));
        }

        Ok(response.value)
    }

    async fn query_abci_with_proofs(
        chain: &Chain,
        path: &str,
        data: &[u8],
        height: &Chain::Height,
    ) -> Result<(Vec<u8>, Vec<u8>), Chain::Error> {
        let response = chain
            .rpc_client()
            .abci_query(
                Some(path.to_owned()),
                data,
                Some(height.clone().into()),
                true,
            )
            .await
            .map_err(Chain::raise_error)?;

        if !response.code.is_ok() {
            return Err(Chain::raise_error(AbciQueryError { response }));
        }

        let raw_proof = response
            .proof
            .ok_or_else(|| Chain::raise_error("empty response proof"))?;

        let proof = convert_tm_to_ics_merkle_proof(&raw_proof).map_err(Chain::raise_error)?;

        let proof_bytes = CommitmentProofBytes::try_from(proof).map_err(Chain::raise_error)?;

        Ok((response.value, proof_bytes.into()))
    }
}
