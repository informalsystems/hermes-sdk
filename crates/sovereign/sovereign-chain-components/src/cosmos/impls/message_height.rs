use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_sovereign_rollup_components::types::height::RollupHeight;

pub struct GetSovereignRollupHeightFromCosmosMessage;

impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetSovereignRollupHeightFromCosmosMessage
where
    Chain: HasMessageType<Message = CosmosMessage>,
    Counterparty: HasHeightType<Height = RollupHeight>,
{
    fn counterparty_message_height_for_update_client(
        message: &CosmosMessage,
    ) -> Option<RollupHeight> {
        let height = message
            .message
            .counterparty_message_height_for_update_client()?;

        if height.revision_number() == 0 {
            Some(RollupHeight {
                slot_number: height.revision_height(),
            })
        } else {
            None
        }
    }
}
