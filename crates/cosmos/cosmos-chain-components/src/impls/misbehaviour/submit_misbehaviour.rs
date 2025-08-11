use alloc::str::FromStr;

use hermes_core::chain_components::traits::{
    HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::Signer;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour;
use prost::{DecodeError, Message};
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
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseAsyncError<DecodeError>,
{
    async fn build_misbehaviour_message(
        _chain: &Chain,
        evidence: &ProstAny,
    ) -> Result<Chain::Message, Chain::Error> {
        let cosmos_evidence = Misbehaviour::decode(&*evidence.value).map_err(Chain::raise_error)?;
        #[allow(deprecated)]
        let client_id = ClientId::from_str(cosmos_evidence.client_id.as_str())
            .expect("Invalid client ID in evidence");
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
