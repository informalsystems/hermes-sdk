use core::convert::Infallible;

use cgp::core::component::UseDelegate;
use cgp::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use eyre::Report;
use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
use hermes_cosmos_core::error::handlers::{
    DisplayError, HandleInfallible, ReportError, ReturnError,
};
use hermes_cosmos_core::error::impls::UseHermesError;
use hermes_cosmos_core::error::types::Error;
use hermes_cosmos_core::ibc::clients::tendermint::types::error::TendermintClientError;
use hermes_cosmos_core::ibc::core::channel::types::error::ChannelError;
use hermes_cosmos_core::ibc::core::host::types::error::{DecodingError, IdentifierError};
use hermes_cosmos_core::runtime::types::error::TokioRuntimeError;
use hermes_prelude::*;
use tonic::transport::Error as TransportError;

pub struct ProvideCliError;

pub struct CliErrorHandlers;

pub trait CanHandleCliError<Context>: ErrorRaiser<Context, TokioRuntimeError>
where
    Context: HasAsyncErrorType<Error = Error>,
{
}

impl<Context> CanHandleCliError<Context> for ProvideCliError where
    Context: HasAsyncErrorType<Error = Error>
{
}

delegate_components! {
    ProvideCliError {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]: UseHermesError,
        ErrorRaiserComponent: UseDelegate<CliErrorHandlers>,
    }
}

delegate_components! {
    CliErrorHandlers {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TokioRuntimeError,
            TendermintClientError,
            IdentifierError,
            ChannelError,
            DecodingError,
            toml::de::Error,
            toml::ser::Error,
            TransportError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
    }
}
