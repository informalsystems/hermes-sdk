use ibc::core::host::types::identifiers::ChainId;

#[derive(Clone, Debug)]
pub struct CosmosProposalSetupClientUpgradeResult {
    pub height: i64,
    pub new_chain_id: ChainId,
}
