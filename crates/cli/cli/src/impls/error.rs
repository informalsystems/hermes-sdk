use core::convert::Infallible;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use eyre::Report;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::impls::ProvideHermesError;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_runtime::types::error::TokioRuntimeError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics02_client::height::HeightError;
use ibc_relayer_types::core::ics03_connection::error::Error as Ics03Error;
use ibc_relayer_types::core::ics23_commitment::error::Error as Ics23Error;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use tonic::transport::Error as TransportError;

pub struct ProvideCliError;

pub struct CliErrorHandlers;

pub trait CanHandleCliError<Context>: ErrorRaiser<Context, TokioRuntimeError>
where
    Context: HasErrorType<Error = Error>,
{
}

impl<Context> CanHandleCliError<Context> for ProvideCliError where
    Context: HasErrorType<Error = Error>
{
}

delegate_components! {
    ProvideCliError {
        [
            ErrorTypeComponent,
            RetryableErrorComponent,
        ]: ProvideHermesError,
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
            toml::de::Error,
            toml::ser::Error,
            HeightError,
            Ics02Error,
            Ics03Error,
            Ics23Error,
            Ics24ValidationError,
            TransportError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
    }
}
