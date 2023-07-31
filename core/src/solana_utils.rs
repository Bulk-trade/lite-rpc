use crate::structures::identity_stakes::IdentityStakes;
use anyhow::Context;
use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, slot_history::Slot};
use solana_streamer::nonblocking::quic::ConnectionPeerType;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::{
    broadcast,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

const AVERAGE_SLOT_CHANGE_TIME_IN_MILLIS: u64 = 400;

pub struct SolanaUtils;

impl SolanaUtils {
    pub async fn get_stakes_for_identity(
        rpc_client: Arc<RpcClient>,
        identity: Pubkey,
    ) -> anyhow::Result<IdentityStakes> {
        let vote_accounts = rpc_client.get_vote_accounts().await?;
        let map_of_stakes: HashMap<String, u64> = vote_accounts
            .current
            .iter()
            .map(|x| (x.node_pubkey.clone(), x.activated_stake))
            .collect();

        if let Some(stakes) = map_of_stakes.get(&identity.to_string()) {
            let all_stakes: Vec<u64> = vote_accounts
                .current
                .iter()
                .map(|x| x.activated_stake)
                .collect();

            let identity_stakes = IdentityStakes {
                peer_type: ConnectionPeerType::Staked,
                stakes: *stakes,
                min_stakes: all_stakes.iter().min().map_or(0, |x| *x),
                max_stakes: all_stakes.iter().max().map_or(0, |x| *x),
                total_stakes: all_stakes.iter().sum(),
            };

            info!(
                "Idenity stakes {}, {}, {}, {}",
                identity_stakes.total_stakes,
                identity_stakes.min_stakes,
                identity_stakes.max_stakes,
                identity_stakes.stakes
            );
            Ok(identity_stakes)
        } else {
            Ok(IdentityStakes::default())
        }
    }

    pub async fn poll_rpc_slots(
        rpc_client: &RpcClient,
        slot_tx: UnboundedSender<Slot>,
    ) -> anyhow::Result<()> {
        let mut poll_frequency = tokio::time::interval(Duration::from_millis(50));

        loop {
            let slot = rpc_client
                .get_slot_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
                    commitment: solana_sdk::commitment_config::CommitmentLevel::Processed,
                })
                .await
                .context("Error getting slot")?;
            // send
            slot_tx.send(slot).context("Error sending slot")?;
            // wait for next poll i.e at least 50ms
            poll_frequency.tick().await;
        }
    }

    // Estimates the slots, either from polled slot or by forcefully updating after every 400ms
    // returns if the estimated slot was updated or not
    pub async fn slot_estimator(
        slot_update_notifier: &mut UnboundedReceiver<u64>,
        mut current_slot: Slot,
        mut estimated_slot: Slot,
    ) -> (Slot, Slot) {
        match tokio::time::timeout(
            Duration::from_millis(AVERAGE_SLOT_CHANGE_TIME_IN_MILLIS),
            slot_update_notifier.recv(),
        )
        .await
        {
            Ok(Some(slot)) => {
                // slot is latest
                if slot > current_slot {
                    current_slot = slot;
                    if current_slot > estimated_slot {
                        estimated_slot = current_slot;
                    }
                }
            }
            Ok(None) => (),
            Err(_) => {
                // force update the slot
                // estimated slot should not go ahead more than 32 slots
                // this is because it may be a slot block
                if estimated_slot < current_slot + 32 {
                    estimated_slot += 1;
                }
            }
        }

        (current_slot, estimated_slot)
    }
}
