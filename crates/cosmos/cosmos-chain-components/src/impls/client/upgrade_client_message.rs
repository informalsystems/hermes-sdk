use hermes_core::chain_components::traits::{
    ClientUpgrade, ClientUpgradeComponent, HasClientIdType, HasHeightType, HasMessageType,
    HasUpgradeClientPayloadType,
};
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_prelude::*;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpgradeClient;

use crate::impls::CosmosUpgradeClientPayload;
use crate::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use crate::types::Secp256k1KeyPair;

pub struct UpgradeClientWithGovernanceProposal;

#[derive(Debug)]
pub struct MsgUpgradeClientProposal {
    pub client_id: String,
    pub client_state: Option<Any>,
    pub consensus_state: Option<Any>,
    pub proof_upgrade_client: Vec<u8>,
    pub proof_upgrade_consensus_state: Vec<u8>,
    pub signer: Signer,
}

#[cgp_provider(ClientUpgradeComponent)]
impl<Chain, Counterparty> ClientUpgrade<Chain, Counterparty> for UpgradeClientWithGovernanceProposal
where
    Chain: HasClientIdType<Counterparty>
        + HasHeightType
        + HasMessageType<Message = CosmosMessage>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + HasAsyncErrorType,
    Counterparty: HasUpgradeClientPayloadType<UpgradeClientPayload = CosmosUpgradeClientPayload>,
{
    async fn upgrade_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        upgrade_client_payload: &CosmosUpgradeClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let signer = chain.get_default_signer().account().into();

        let msg = MsgUpgradeClientProposal {
            client_id: client_id.to_string(),
            client_state: Some(upgrade_client_payload.upgrade_client_state.clone()),
            consensus_state: Some(upgrade_client_payload.upgrade_consensus_state.clone()),
            proof_upgrade_client: upgrade_client_payload.upgrade_client_state_proof.clone(),
            proof_upgrade_consensus_state: upgrade_client_payload
                .upgrade_consensus_state_proof
                .clone(),
            signer,
        };

        Ok(msg.to_cosmos_message())
    }
}

impl DynCosmosMessage for MsgUpgradeClientProposal {
    fn encode_protobuf(&self, _signer: &Signer) -> Any {
        let proposal_message = MsgUpgradeClient {
            client_id: self.client_id.clone(),
            client_state: self.client_state.clone(),
            consensus_state: self.consensus_state.clone(),
            proof_upgrade_client: self.proof_upgrade_client.clone(),
            proof_upgrade_consensus_state: self.proof_upgrade_consensus_state.clone(),
            signer: self.signer.to_string(),
        };

        Any::from_msg(&proposal_message).expect("failed to convert `MsgUpgradeClient` to `Any`")
    }
}
