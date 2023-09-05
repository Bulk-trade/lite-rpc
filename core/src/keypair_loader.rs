use solana_sdk::signature::Keypair;
use std::env;

// note this is duplicated from lite-rpc module
pub async fn load_identity_keypair(identity_from_cli: &String) -> Option<Keypair> {
    if let Ok(identity_env_var) = env::var("IDENTITY") {
        if let Ok(identity_bytes) = serde_json::from_str::<Vec<u8>>(identity_env_var.as_str()) {
            Some(Keypair::from_bytes(identity_bytes.as_slice()).unwrap())
        } else {
            // must be a file
            let identity_file = tokio::fs::read_to_string(identity_env_var.as_str())
                .await
                .expect("Cannot find the identity file provided");
            let identity_bytes: Vec<u8> = serde_json::from_str(&identity_file).unwrap();
            Some(Keypair::from_bytes(identity_bytes.as_slice()).unwrap())
        }
    } else if identity_from_cli.is_empty() {
        None
    } else {
        let identity_file = tokio::fs::read_to_string(identity_from_cli.as_str())
            .await
            .expect("Cannot find the identity file provided");
        let identity_bytes: Vec<u8> = serde_json::from_str(&identity_file).unwrap();
        Some(Keypair::from_bytes(identity_bytes.as_slice()).unwrap())
    }
}
