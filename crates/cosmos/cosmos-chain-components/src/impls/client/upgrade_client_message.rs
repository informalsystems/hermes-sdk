use hermes_core::chain_components::traits::{
    ClientUpgrade, ClientUpgradeComponent, HasClientIdType, HasMessageType,
    HasUpgradeClientPayloadType,
};
use hermes_prelude::*;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpgradeClient;

use crate::impls::CosmosUpgradeClientPayload;
use crate::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use crate::types::{TendermintClientState, TendermintConsensusState};

pub struct UpgradeClientWithGovernanceProposal;

#[derive(Debug)]
pub struct MsgUpgradeClientProposal {
    pub client_id: String,
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
    pub proof_upgrade_client: Vec<u8>,
    pub proof_upgrade_consensus_state: Vec<u8>,
    pub signer: Signer,
}

#[cgp_provider(ClientUpgradeComponent)]
impl<Chain, Counterparty> ClientUpgrade<Chain, Counterparty> for UpgradeClientWithGovernanceProposal
where
    Chain: HasClientIdType<Counterparty>
        + HasUpgradeClientPayloadType<UpgradeClientPayload = CosmosUpgradeClientPayload>
        + HasMessageType<Message = CosmosMessage>,
{
    async fn upgrade_client_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        upgrade_client_payload: &CosmosUpgradeClientPayload,
    ) -> CosmosMessage {
        let msg = MsgUpgradeClientProposal {
            client_id: client_id.to_string(),
            client_state: upgrade_client_payload.client_state.clone(),
            consensus_state: upgrade_client_payload.consensus_state.clone(),
            proof_upgrade_client: upgrade_client_payload.proof_upgrade_client.clone(),
            proof_upgrade_consensus_state: upgrade_client_payload
                .proof_upgrade_consensus_state
                .clone(),
            signer: upgrade_client_payload.signer.clone(),
        };

        msg.to_cosmos_message()
    }
}

impl DynCosmosMessage for MsgUpgradeClientProposal {
    fn encode_protobuf(&self, _signer: &Signer) -> Any {
        let proposal_message = MsgUpgradeClient {
            client_id: self.client_id.clone(),
            client_state: Some(self.client_state.clone().into()),
            consensus_state: Some(self.consensus_state.clone().into()),
            proof_upgrade_client: self.proof_upgrade_client.clone(),
            proof_upgrade_consensus_state: self.proof_upgrade_consensus_state.clone(),
            signer: self.signer.to_string(),
        };

        Any::from_msg(&proposal_message).expect("failed to convert `MsgUpgradeClient` to `Any`")
    }
}
