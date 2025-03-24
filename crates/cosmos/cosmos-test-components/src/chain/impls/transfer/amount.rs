use core::marker::PhantomData;
use std::string::FromUtf8Error;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_test_components::chain::traits::transfer::amount::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc::core::host::types::identifiers::{ChannelId, PortId};
use sha2::{Digest, Sha256};
use subtle_encoding::hex;

use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

pub struct ConvertCosmosIbcAmount;

#[cgp_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertCosmosIbcAmount
where
    Chain: HasAmountType<Amount = Amount, Denom = Denom>
        + CanRaiseAsyncError<FromUtf8Error>
        + HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>,
    Counterparty: HasAmountType<Amount = Amount>,
{
    async fn ibc_transfer_amount_from(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Amount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<Amount, Chain::Error> {
        let denom = derive_ibc_denom(port_id, channel_id, &counterparty_amount.denom)
            .map_err(Chain::raise_error)?;

        Ok(Amount {
            quantity: counterparty_amount.quantity,
            denom,
        })
    }

    async fn transmute_counterparty_amount(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Amount,
        denom: &Denom,
    ) -> Result<Amount, Chain::Error> {
        Ok(Amount {
            quantity: counterparty_amount.quantity,
            denom: denom.clone(),
        })
    }
}

pub fn derive_ibc_denom(
    port_id: &PortId,
    channel_id: &ChannelId,
    denom: &Denom,
) -> Result<Denom, FromUtf8Error> {
    fn derive_denom(
        port_id: &PortId,
        channel_id: &ChannelId,
        denom: &str,
    ) -> Result<String, FromUtf8Error> {
        let transfer_path = format!("{port_id}/{channel_id}/{denom}");
        derive_denom_with_path(&transfer_path)
    }

    /// Derive the transferred token denomination using
    /// <https://github.com/cosmos/ibc-go/blob/main/docs/architecture/adr-001-coin-source-tracing.md>
    fn derive_denom_with_path(transfer_path: &str) -> Result<String, FromUtf8Error> {
        let mut hasher = Sha256::new();
        hasher.update(transfer_path.as_bytes());

        let denom_bytes = hasher.finalize();
        let denom_hex = String::from_utf8(hex::encode_upper(denom_bytes))?;

        Ok(format!("ibc/{denom_hex}"))
    }

    match denom {
        Denom::Base(denom) => {
            let hashed = derive_denom(port_id, channel_id, denom)?;

            Ok(Denom::Ibc {
                path: format!("{port_id}/{channel_id}"),
                denom: denom.clone(),
                hashed,
            })
        }
        Denom::Ibc { path, denom, .. } => {
            let new_path = format!("{port_id}/{channel_id}/{path}");
            let hashed = derive_denom_with_path(&format!("{new_path}/{denom}"))?;

            Ok(Denom::Ibc {
                path: new_path,
                denom: denom.clone(),
                hashed,
            })
        }
    }
}
