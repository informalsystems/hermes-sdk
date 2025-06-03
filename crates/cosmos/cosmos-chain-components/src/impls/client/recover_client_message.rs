use hermes_core::chain_components::traits::{
    ClientRecovery, ClientRecoveryComponent, HasClientIdType, HasMessageType,
    HasRecoverClientPayloadType,
};
use hermes_prelude::*;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::gov::v1::MsgSubmitProposal;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgRecoverClient;
use prost::Message;

use crate::impls::client::CosmosRecoverClientPayload;
use crate::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};

pub struct RecoverClientWithGovernanceProposal;

#[derive(Debug)]
pub struct MsgRecoverClientProposal {
    pub proposal_message: MsgRecoverClient,
    pub title: String,
    pub summary: String,
    pub deposit_amount: u128,
    pub deposit_denom: String,
}

#[cgp_provider(ClientRecoveryComponent)]
impl<Chain, Counterparty> ClientRecovery<Chain, Counterparty>
    for RecoverClientWithGovernanceProposal
where
    Chain: HasClientIdType<Counterparty>
        + HasMessageType<Message = CosmosMessage>
        + HasRecoverClientPayloadType<RecoverClientPayload = CosmosRecoverClientPayload>,
{
    async fn recover_client_message(
        _chain: &Chain,
        subject_client: &Chain::ClientId,
        substitute_client: &Chain::ClientId,
        recover_client_payload: &CosmosRecoverClientPayload,
    ) -> CosmosMessage {
        let proposal_message = MsgRecoverClient {
            subject_client_id: subject_client.to_string(),
            substitute_client_id: substitute_client.to_string(),
            signer: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".to_owned(), // TODO: Do not hardcore this
        };

        let msg = MsgRecoverClientProposal {
            proposal_message,
            title: "Recover Client Proposal".into(),
            summary: format!("Proposal to recover expired client {subject_client}"),
            deposit_amount: recover_client_payload.deposit_amount,
            deposit_denom: recover_client_payload.deposit_denom.clone(),
        };

        msg.to_cosmos_message()
    }
}

impl DynCosmosMessage for MsgRecoverClientProposal {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proposal_message = MsgSubmitProposal {
            messages: vec![Any {
                type_url: "/ibc.core.client.v1.MsgRecoverClient".into(),
                value: self.proposal_message.encode_to_vec(),
            }],
            initial_deposit: vec![Coin {
                denom: self.deposit_denom.clone(),
                amount: self.deposit_amount.to_string(),
            }],
            proposer: signer.to_string(),
            metadata: "".into(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            expedited: false,
        };

        Any {
            type_url: "/cosmos.gov.v1.MsgSubmitProposal".into(),
            value: proposal_message.encode_to_vec(),
        }
    }
}
