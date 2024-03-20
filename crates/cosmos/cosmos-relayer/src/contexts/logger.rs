use cgp_core::prelude::*;
use hermes_relayer_components::log::impls::delegate::DelegateLogger;
use hermes_relayer_components::log::traits::logger::LoggerComponent;
use serde::Serialize;

#[derive(Serialize)]
pub struct CosmosLogger;

pub struct CosmosLoggerComponents;

impl HasComponents for CosmosLogger {
    type Components = CosmosLoggerComponents;
}

delegate_components! {
    CosmosLoggerComponents {
        LoggerComponent: DelegateLogger<CosmosLoggingDelegates>,
    }
}

pub struct CosmosLoggingDelegates;
