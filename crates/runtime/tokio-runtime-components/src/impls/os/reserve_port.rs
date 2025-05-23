use std::io::Error as IoError;
use std::net::{Ipv4Addr, SocketAddrV4};

use hermes_prelude::*;
use hermes_runtime_components::traits::{TcpPortReserver, TcpPortReserverComponent};
use rand::Rng;
use tokio::net::TcpListener;

pub struct TokioReserveTcpPort;

#[cgp_provider(TcpPortReserverComponent)]
impl<Runtime> TcpPortReserver<Runtime> for TokioReserveTcpPort
where
    Runtime: Async + CanRaiseAsyncError<IoError>,
{
    async fn reserve_tcp_port(_runtime: &Runtime) -> Result<u16, Runtime::Error> {
        // TODO: abort if failed to find a free port after X tries
        loop {
            let port = {
                let mut rng = rand::thread_rng();
                rng.gen_range(1025..=u16::MAX)
            };

            let loopback = Ipv4Addr::new(127, 0, 0, 1);
            let address = SocketAddrV4::new(loopback, port);

            if TcpListener::bind(address).await.is_ok() {
                // TODO: also check if the port has been previously reserved

                return Ok(port);
            }
        }
    }
}
