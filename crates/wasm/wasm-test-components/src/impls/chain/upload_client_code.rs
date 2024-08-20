use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;

use crate::traits::chain::store_code::CanBuildStoreCodeMessage;
use crate::traits::chain::upload_client_code::WasmClientCodeUploader;

pub struct SendStoreCodeProposalMessage;

impl<Chain> WasmClientCodeUploader<Chain> for SendStoreCodeProposalMessage
where
    Chain: CanBuildStoreCodeMessage + CanSendSingleMessage,
{
    async fn upload_wasm_client_code(
        chain: &Chain,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        deposit_amount: &Chain::Amount,
    ) -> Result<(), Chain::Error> {
        let message =
            chain.build_store_code_message(wasm_client_bytes, title, summary, deposit_amount);

        chain.send_message(message).await?;

        Ok(())
    }
}
