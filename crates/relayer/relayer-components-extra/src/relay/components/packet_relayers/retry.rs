use core::marker::PhantomData;

use cgp_core::{async_trait, HasErrorType};
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::components::packet_relayer::PacketRelayer;
use hermes_relayer_components::relay::types::aliases::Packet;

pub struct RetryRelayer<InRelay> {
    pub phantom: PhantomData<InRelay>,
}

pub trait SupportsPacketRetry: HasErrorType {
    const MAX_RETRY: usize;

    fn is_retryable_error(e: &Self::Error) -> bool;

    fn max_retry_exceeded_error(e: Self::Error) -> Self::Error;
}

#[async_trait]
impl<Relay, InRelayer> PacketRelayer<Relay> for RetryRelayer<InRelayer>
where
    Relay: HasRelayChains,
    InRelayer: PacketRelayer<Relay>,
    Relay: SupportsPacketRetry,
{
    async fn relay_packet(context: &Relay, packet: &Packet<Relay>) -> Result<(), Relay::Error> {
        let mut retries_made: usize = 0;
        loop {
            let res = InRelayer::relay_packet(context, packet).await;

            match res {
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    if Relay::is_retryable_error(&e) {
                        retries_made += 1;

                        if retries_made > Relay::MAX_RETRY {
                            return Err(Relay::max_retry_exceeded_error(e));
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }
}
