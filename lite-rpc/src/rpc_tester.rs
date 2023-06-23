use std::net::SocketAddr;

use lite_rpc::cli::Args;
use prometheus::{opts, register_gauge, Gauge};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

lazy_static::lazy_static! {
    static ref RPC_RESPONDING: Gauge =
    register_gauge!(opts!("literpc_rpc_responding", "If LiteRpc is responding")).unwrap();
}

pub struct RpcTester(RpcClient);

impl From<&Args> for RpcTester {
    fn from(value: &Args) -> Self {
        let addr: SocketAddr = value
            .lite_rpc_http_addr
            .parse()
            .expect("Invalid literpc http address");

        RpcTester(RpcClient::new(format!("http://0.0.0.0:{}", addr.port())))
    }
}

impl RpcTester {
    /// Starts a loop that checks if the rpc is responding every 5 seconds
    pub async fn start(self) -> ! {
        loop {
            // sleep for 10 seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            // do a simple request to self for getVersion
            let Err(err) = self.0.get_version().await else {
                RPC_RESPONDING.set(1.0);
                continue;
            };

            RPC_RESPONDING.set(0.0);
            log::error!("Rpc not responding {err:?}");
        }
    }
}
