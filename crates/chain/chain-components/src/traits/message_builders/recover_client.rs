use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasMessageType};
use hermes_prelude::*;

use crate::traits::{ClientRecovery, ClientRecoveryComponent, HasRecoverClientPayloadType};

#[cgp_provider(ClientRecoveryComponent)]
impl<Chain, Counterparty, Components, Delegate> ClientRecovery<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasRecoverClientPayloadType + HasMessageType,
    Delegate: ClientRecovery<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn recover_client_message(
        chain: &Chain,
        subject_client: &Chain::ClientId,
        substitute_client: &Chain::ClientId,
        recover_client_payload: &Chain::RecoverClientPayload,
    ) -> Chain::Message {
        Delegate::recover_client_message(
            chain,
            subject_client,
            substitute_client,
            recover_client_payload,
        )
        .await
    }
}
