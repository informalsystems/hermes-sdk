use hermes_core::chain_components::traits::{
    ClientUpgradePayloadBuilder, ClientUpgradePayloadBuilderComponent, HasCommitmentProofBytes,
    HasHeightType, HasUpgradeClientPayloadType,
};
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::cosmos_host::SDK_UPGRADE_QUERY_PATH;
use ibc_proto::google::protobuf::Any;
use prost::{DecodeError, Message};

use crate::traits::CanQueryAbci;

#[derive(Clone, Debug)]
pub struct CosmosUpgradeClientPayload {
    pub upgrade_height: Height,
    pub upgrade_client_state: Any,
    pub upgrade_consensus_state: Any,
    pub upgrade_client_state_proof: Vec<u8>,
    pub upgrade_consensus_state_proof: Vec<u8>,
}

pub struct CosmosClientUpgradePayloadBuilder;

#[cgp_provider(ClientUpgradePayloadBuilderComponent)]
impl<Chain, Counterparty> ClientUpgradePayloadBuilder<Chain, Counterparty>
    for CosmosClientUpgradePayloadBuilder
where
    Chain: HasUpgradeClientPayloadType<UpgradeClientPayload = CosmosUpgradeClientPayload>
        + HasHeightType<Height = Height>
        + CanQueryAbci
        + HasCommitmentProofBytes
        + CanRaiseAsyncError<DecodeError>,
{
    async fn upgrade_client_payload(
        chain: &Chain,
        upgrade_height: &Height,
    ) -> Result<CosmosUpgradeClientPayload, Chain::Error> {
        let client_state_upgrade_path = format!(
            "upgradedIBCState/{}/upgradedClient",
            upgrade_height.revision_height()
        );
        let query_height = upgrade_height.decrement().unwrap();

        let (upgrade_client_state, raw_upgrade_client_state_proof) = chain
            .query_abci_with_proofs(
                SDK_UPGRADE_QUERY_PATH,
                client_state_upgrade_path.into_bytes().as_slice(),
                &query_height,
            )
            .await?;
        let client_state_any: Any =
            Message::decode(upgrade_client_state.unwrap().as_ref()).map_err(Chain::raise_error)?;

        let upgrade_client_state_proof =
            Chain::commitment_proof_bytes(&raw_upgrade_client_state_proof);

        let consensus_state_upgrade_path = format!(
            "upgradedIBCState/{}/upgradedConsState",
            upgrade_height.revision_height()
        );
        let (upgrade_consensus_state, raw_upgrade_consensus_state_proof) = chain
            .query_abci_with_proofs(
                SDK_UPGRADE_QUERY_PATH,
                consensus_state_upgrade_path.into_bytes().as_slice(),
                &query_height,
            )
            .await?;
        let consensus_state_any: Any = Message::decode(upgrade_consensus_state.unwrap().as_ref())
            .map_err(Chain::raise_error)?;

        let upgrade_consensus_state_proof =
            Chain::commitment_proof_bytes(&raw_upgrade_consensus_state_proof);

        Ok(CosmosUpgradeClientPayload {
            upgrade_height: *upgrade_height,
            upgrade_client_state: client_state_any,
            upgrade_consensus_state: consensus_state_any,
            upgrade_client_state_proof: upgrade_client_state_proof.to_vec(),
            upgrade_consensus_state_proof: upgrade_consensus_state_proof.to_vec(),
        })
    }
}
