use hermes_core::chain_components::traits::{
    HasEvidenceFields, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour;
use prost_types::Any as ProstAny;

use crate::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};

#[derive(Debug)]
pub struct SubmitMisbehaviour {
    pub client_id: ClientId,
    pub evidence: ProstAny,
}

pub struct TendermintMisbehaviourMessageBuilder;

#[cgp_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty> MisbehaviourMessageBuilder<Chain, Counterparty>
    for TendermintMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = ProstAny>
        + HasEvidenceFields<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = CosmosMessage>
        + HasAsyncErrorType,
{
    async fn build_misbehaviour_message(
        _chain: &Chain,
        evidence: &ProstAny,
    ) -> Result<Chain::Message, Chain::Error> {
        let client_id = Chain::evidence_client_id(evidence);
        let msg = SubmitMisbehaviour {
            client_id: client_id.clone(),
            evidence: evidence.clone(),
        };

        Ok(msg.to_cosmos_message())
    }
}

impl DynCosmosMessage for SubmitMisbehaviour {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let misbehaviour_message = MsgSubmitMisbehaviour {
            client_id: self.client_id.to_string(),
            misbehaviour: Some(Any {
                type_url: self.evidence.type_url.clone(),
                value: self.evidence.value.clone(),
            }),
            signer: signer.to_string(),
        };

        Any::from_msg(&misbehaviour_message)
            .expect("failed to convert `MsgSubmitMisbehaviour` to `Any`")
    }
}
