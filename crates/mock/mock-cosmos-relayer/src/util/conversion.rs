use ibc::core::commitment_types::merkle::MerkleProof;
use ibc::core::commitment_types::proto::ics23::CommitmentProof;
use ibc::core::commitment_types::proto::v1::MerkleProof as RawMerkleProof;
use tendermint::merkle::proof::ProofOps;

use crate::types::error::Error;

pub fn convert_tm_to_ics_merkle_proof(tm_proof: &ProofOps) -> Result<MerkleProof, Error> {
    let mut proofs = Vec::new();

    for op in &tm_proof.ops {
        let mut parsed = CommitmentProof { proof: None };

        prost::Message::merge(&mut parsed, op.data.as_slice()).map_err(Error::source)?;

        proofs.push(parsed);
    }

    let merkle_proof = MerkleProof::try_from(RawMerkleProof { proofs }).map_err(Error::source)?;

    Ok(merkle_proof)
}
