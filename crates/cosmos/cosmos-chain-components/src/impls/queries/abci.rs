use cgp::prelude::*;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::merkle::MerkleProof;
use ics23::CommitmentProof;
use prost::{DecodeError, Message};
use tendermint::block::Height as TendermintHeight;
use tendermint::merkle::proof::ProofOps as TendermintProof;
use tendermint::Error as TendermintError;
use tendermint_rpc::endpoint::abci_query::AbciQuery;
use tendermint_rpc::{Client, Error as RpcError};

use crate::components::client::AbciQuerierComponent;
use crate::traits::abci_query::AbciQuerier;
use crate::traits::rpc_client::HasRpcClient;
use crate::types::commitment_proof::CosmosCommitmentProof;

pub struct QueryAbci;

#[derive(Debug)]
pub struct AbciQueryError {
    pub response: AbciQuery,
}

#[cgp_provider(AbciQuerierComponent)]
impl<Chain, Encoding> AbciQuerier<Chain> for QueryAbci
where
    Chain: HasRpcClient
        + HasHeightType<Height = Height>
        + HasEncoding<AsBytes, Encoding = Encoding>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + CanRaiseAsyncError<RpcError>
        + CanRaiseAsyncError<AbciQueryError>
        + CanRaiseAsyncError<TendermintError>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<&'static str>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanEncode<ViaProtobuf, MerkleProof>,
{
    async fn query_abci(
        chain: &Chain,
        path: &str,
        data: &[u8],
        height: &Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let tm_height =
            TendermintHeight::try_from(height.revision_height()).map_err(Chain::raise_error)?;
        let response = chain
            .rpc_client()
            .abci_query(Some(path.to_owned()), data, Some(tm_height), false)
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
        query_height: &Height,
    ) -> Result<(Vec<u8>, Chain::CommitmentProof), Chain::Error> {
        let tm_height = TendermintHeight::try_from(query_height.revision_height())
            .map_err(Chain::raise_error)?;
        let response = chain
            .rpc_client()
            .abci_query(Some(path.to_owned()), data, Some(tm_height), true)
            .await
            .map_err(Chain::raise_error)?;

        if !response.code.is_ok() {
            return Err(Chain::raise_error(AbciQueryError { response }));
        }

        let raw_proof = response
            .proof
            .ok_or_else(|| Chain::raise_error("empty response proof"))?;

        let merkle_proof =
            convert_tm_to_ics_merkle_proof(&raw_proof).map_err(Chain::raise_error)?;

        let proof_bytes = chain
            .encoding()
            .encode(&merkle_proof)
            .map_err(Chain::raise_error)?;

        let proof_height = query_height.add(1);

        let commitment_proof = CosmosCommitmentProof {
            merkle_proof,
            proof_bytes,
            proof_height,
        };

        Ok((response.value, commitment_proof))
    }
}

pub fn convert_tm_to_ics_merkle_proof(
    tm_proof: &TendermintProof,
) -> Result<MerkleProof, DecodeError> {
    let mut proofs = Vec::new();

    for op in &tm_proof.ops {
        let parsed: CommitmentProof = Message::decode(op.data.as_slice())?;

        proofs.push(parsed);
    }

    Ok(MerkleProof { proofs })
}
