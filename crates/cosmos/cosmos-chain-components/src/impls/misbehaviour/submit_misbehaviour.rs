use hermes_core::chain_components::traits::{
    HasEvidenceFields, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::Signer;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour;

use crate::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};

#[derive(Debug)]
pub struct SubmitMisbehaviour {
    pub client_id: ClientId,
    pub evidence: Misbehaviour,
}

pub struct TendermintMisbehaviourMessageBuilder;

#[cgp_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty> MisbehaviourMessageBuilder<Chain, Counterparty>
    for TendermintMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = Misbehaviour>
        + HasEvidenceFields<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + HasAsyncErrorType,
{
    async fn build_misbehaviour_message(
        _chain: &Chain,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        let msg = SubmitMisbehaviour {
            client_id: Chain::evidence_client_id(evidence),
            evidence: evidence.clone(),
        };

        Ok(msg.to_cosmos_message())
    }
}

impl DynCosmosMessage for SubmitMisbehaviour {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let misbehaviour_message = MsgSubmitMisbehaviour {
            client_id: self.client_id.to_string(),
            misbehaviour: Some(
                Any::from_msg(&self.evidence).expect("failed to convert `Misbehaviour` to `Any`"),
            ),
            signer: signer.to_string(),
        };

        Any::from_msg(&misbehaviour_message)
            .expect("failed to convert `MsgSubmitMisbehaviour` to `Any`")
    }
}
