use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_protobuf_encoding_components::types::Protobuf;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_relayer_types::core::ics23_commitment::error::Error as Ics23Error;
use ibc_relayer_types::proofs::ProofError;
use ibc_relayer_types::Height;
use ics23::CommitmentProof;
use prost::{DecodeError, Message};
use tendermint::merkle::proof::ProofOps as TendermintProof;
use tendermint_rpc::endpoint::abci_query::AbciQuery;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::abci_query::AbciQuerier;
use crate::traits::rpc_client::HasRpcClient;
use crate::types::commitment_proof::CosmosCommitmentProof;

pub struct QueryAbci;

#[derive(Debug)]
pub struct AbciQueryError {
    pub response: AbciQuery,
}

impl<Chain, Encoding> AbciQuerier<Chain> for QueryAbci
where
    Chain: HasRpcClient
        + HasHeightType<Height = Height>
        + HasEncoding<Encoding = Encoding>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + CanRaiseError<RpcError>
        + CanRaiseError<AbciQueryError>
        + CanRaiseError<Ics23Error>
        + CanRaiseError<ProofError>
        + CanRaiseError<DecodeError>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<&'static str>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanEncode<Protobuf, MerkleProof>,
{
    async fn query_abci(
        chain: &Chain,
        path: &str,
        data: &[u8],
        height: &Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let response = chain
            .rpc_client()
            .abci_query(Some(path.to_owned()), data, Some((*height).into()), false)
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
        let response = chain
            .rpc_client()
            .abci_query(
                Some(path.to_owned()),
                data,
                Some((*query_height).into()),
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

        let merkle_proof =
            convert_tm_to_ics_merkle_proof(&raw_proof).map_err(Chain::raise_error)?;

        let proof_bytes = chain
            .encoding()
            .encode(&merkle_proof)
            .map_err(Chain::raise_error)?;

        let proof_height = *query_height + 1;

        // println!(
        //     "built proof from ABCI query for {}: {:?}",
        //     core::str::from_utf8(data).unwrap(),
        //     merkle_proof
        // );

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
