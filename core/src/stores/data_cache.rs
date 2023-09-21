use std::sync::{atomic::AtomicU64, Arc};

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::slot_history::Slot;

use crate::{
    stores::{
        block_information_store::BlockInformationStore, cluster_info_store::ClusterInfo,
        subscription_store::SubscriptionStore, tx_store::TxStore,
    },
    structures::{
        identity_stakes::IdentityStakes,
        slot_notification::{AtomicSlot, SlotNotification},
    },
};
pub type TxSubKey = (String, CommitmentConfig);

#[derive(Default, Clone)]
pub struct SlotCache {
    current_slot: AtomicSlot,
    estimated_slot: AtomicSlot,
}

/// The central data store for all data from the cluster.
#[derive(Clone)]
pub struct DataCache {
    pub block_store: BlockInformationStore,
    pub txs: TxStore,
    pub tx_subs: SubscriptionStore,
    pub slot_cache: SlotCache,
    pub identity_stakes: IdentityStakes,
    pub cluster_info: ClusterInfo,
}

impl DataCache {
    pub async fn clean(&self, ttl_duration: std::time::Duration) {
        let block_info = self
            .block_store
            .get_latest_block_info(CommitmentConfig::finalized())
            .await;
        self.block_store.clean().await;
        self.txs.clean(block_info.block_height);

        self.tx_subs.clean(ttl_duration);
    }
}

impl SlotCache {
    pub fn new(slot: Slot) -> Self {
        Self {
            current_slot: Arc::new(AtomicU64::new(slot)),
            estimated_slot: Arc::new(AtomicU64::new(slot)),
        }
    }
    pub fn get_current_slot(&self) -> Slot {
        self.current_slot.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_estimated_slot(&self) -> Slot {
        self.estimated_slot
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn update(&self, slot_notification: SlotNotification) {
        self.current_slot.store(
            slot_notification.processed_slot,
            std::sync::atomic::Ordering::Relaxed,
        );
        self.estimated_slot.store(
            slot_notification.estimated_processed_slot,
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}
