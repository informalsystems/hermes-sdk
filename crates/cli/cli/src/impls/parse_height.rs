use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_cli_components::traits::parse::{ArgParser, CanParseArg};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;

pub struct ParseCosmosHeight<ChainIdTag>(pub PhantomData<ChainIdTag>);

impl<App, Args, ChainIdTag, HeightTag> ArgParser<App, Args, HeightTag>
    for ParseCosmosHeight<ChainIdTag>
where
    App: CanParseArg<Args, ChainIdTag, Parsed = ChainId> + CanRaiseError<Ics02Error>,
    Args: HasField<HeightTag, Field = Option<u64>>,
    ChainIdTag: Async,
{
    type Parsed = Option<Height>;

    fn parse_arg(
        app: &App,
        args: &Args,
        _tag: PhantomData<HeightTag>,
    ) -> Result<Option<Height>, App::Error> {
        match args.get_field(PhantomData) {
            Some(raw_height) => {
                let chain_id = app.parse_arg(args, PhantomData)?;

                let height =
                    Height::new(chain_id.version(), *raw_height).map_err(App::raise_error)?;

                Ok(Some(height))
            }
            None => Ok(None),
        }
    }
}
