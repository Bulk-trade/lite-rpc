use const_env::from_env;
use solana_transaction_status::TransactionConfirmationStatus;

pub mod bridge;
pub mod cli;
pub mod configs;
pub mod encoding;
pub mod errors;
pub mod jsonrpsee_subscrption_handler_sink;
pub mod postgres;
pub mod rpc;

#[from_env]
pub const DEFAULT_RPC_ADDR: &str = "http://0.0.0.0:8899";
#[from_env]
pub const DEFAULT_LITE_RPC_ADDR: &str = "http://0.0.0.0:8890";
#[from_env]
pub const DEFAULT_WS_ADDR: &str = "ws://0.0.0.0:8900";

#[from_env]
pub const DEFAULT_MAX_NUMBER_OF_TXS_IN_QUEUE: usize = 200_000;


#[from_env]
pub const MAX_RETRIES: u16 = 40;

pub const DEFAULT_RETRY_TIMEOUT: u64 = 2;

#[from_env]
pub const DEFAULT_CLEAN_INTERVAL_MS: u64 = 5 * 60 * 1000; // five minute
pub const DEFAULT_TRANSACTION_CONFIRMATION_STATUS: TransactionConfirmationStatus =
    TransactionConfirmationStatus::Finalized;
