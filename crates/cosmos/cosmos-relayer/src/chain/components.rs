use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::components::client::*;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::*;
use hermes_cosmos_chain_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_chain_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::with_cosmos_tx_components;
use hermes_cosmos_test_components::chain::components::*;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::IbcCommitmentPrefixGetter;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use ibc_relayer::keyring::Secp256k1KeyPair;

use crate::contexts::chain::CosmosChain;
use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::contexts::logger::ProvideCosmosLogger;
use crate::impls::error::HandleCosmosError;

pub struct CosmosChainComponents;

impl HasComponents for CosmosChainComponents {
    type Components = CosmosBaseChainComponents;
}

impl HasComponents for CosmosChain {
    type Components = CosmosChainComponents;
}

delegate_components! {
    CosmosChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideCosmosLogger,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideCosmosEncoding,
    }
}

with_cosmos_client_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmosClientComponents: CosmosClientComponents,
        }
    }
}

with_cosmos_tx_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmosTxComponents : CosmosTxComponents,
        }
    }
}

with_cosmmos_chain_test_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmmosChainTestComponents: CosmmosChainTestComponents,
        }
    }
}

pub struct CosmosBaseChainComponents;

delegate_components! {
    CosmosBaseChainComponents {
        [
            ChainStatusQuerierComponent,
            ConsensusStateQuerierComponent,
        ]:
            CosmosClientComponents,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents,
    }
}

impl TxExtensionOptionsGetter<CosmosChain> for CosmosChainComponents {
    fn tx_extension_options(chain: &CosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<CosmosChain> for CosmosChainComponents {
    fn gas_config(chain: &CosmosChain) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

impl DefaultSignerGetter<CosmosChain> for CosmosChainComponents {
    fn get_default_signer(chain: &CosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<CosmosChain> for CosmosChainComponents {
    fn fee_for_simulation(chain: &CosmosChain) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl ProvideMutexForNonceAllocation<CosmosChain> for CosmosChainComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a CosmosChain,
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

impl IbcCommitmentPrefixGetter<CosmosChain> for CosmosChainComponents {
    fn ibc_commitment_prefix(chain: &CosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}
