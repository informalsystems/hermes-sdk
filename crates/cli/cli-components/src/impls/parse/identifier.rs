use core::marker::PhantomData;

use cgp::prelude::*;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::PortId;

use crate::traits::parse::ArgParser;

const TRANSFER_PORT_ID: &str = "transfer";

pub struct ParsePortId<Parsed>(pub PhantomData<Parsed>);

impl<App, Args, Tag, Parsed> ArgParser<App, Args, Tag> for ParsePortId<Parsed>
where
    App: CanRaiseAsyncError<IdentifierError>,
    Args: HasField<Tag, Value = String>,
    Parsed: Async,
{
    type Parsed = PortId;

    fn parse_arg(_app: &App, args: &Args, _tag: PhantomData<Tag>) -> Result<Parsed, App::Error> {
        let port_id: PortId = args
            .get_field(PhantomData)
            .parse()
            .map_err(App::raise_error)?;

        if port_id.validate().is_err() {
            PortId::new(TRANSFER_PORT_ID.to_string())
        } else {
            Ok(port_id)
        }
    }
}
