use futures::lock::Mutex;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::runtime::traits::mutex::MutexGuardOf;
use hermes_relayer_components::transaction::traits::fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::nonce::mutex::HasMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::signer::DefaultSignerGetter;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::components::CosmosTxComponents;

impl ChainIdGetter<CosmosTxContext> for CosmosTxComponents {
    fn chain_id(context: &CosmosTxContext) -> &ChainId {
        &context.tx_config.chain_id
    }
}

impl DefaultSignerGetter<CosmosTxContext> for CosmosTxComponents {
    fn get_default_signer(chain: &CosmosTxContext) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<CosmosTxContext> for CosmosTxComponents {
    fn fee_for_simulation(chain: &CosmosTxContext) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl HasMutexForNonceAllocation for CosmosTxContext {
    fn mutex_for_nonce_allocation(&self, _signer: &Secp256k1KeyPair) -> &Mutex<()> {
        &self.nonce_mutex
    }

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, Self::Runtime, ()>,
        nonce: Self::Nonce,
    ) -> Self::NonceGuard<'a> {
        (mutex_guard, nonce)
    }
}
