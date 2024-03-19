use futures::lock::Mutex;
use hermes_cosmos_client_components::types::nonce_guard::NonceGuard;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::runtime::traits::mutex::MutexGuardOf;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::transaction::traits::fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::nonce::mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::signer::DefaultSignerGetter;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::components::CosmosTxComponents;

impl ProvideRuntime<CosmosTxContext> for CosmosTxComponents {
    fn runtime(chain: &CosmosTxContext) -> &HermesRuntime {
        &chain.runtime
    }
}

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

impl ProvideMutexForNonceAllocation<CosmosTxContext> for CosmosTxComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a CosmosTxContext,
        _signer: &Secp256k1KeyPair,
    ) -> &'a Mutex<()> {
        &chain.nonce_mutex
    }

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, HermesRuntime, ()>,
        account: Account,
    ) -> NonceGuard<'a> {
        NonceGuard {
            mutex_guard,
            account,
        }
    }
}
