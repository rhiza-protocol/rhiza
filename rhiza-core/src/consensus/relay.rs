use crate::crypto::{Hash, PublicKey, Signature};
use crate::crypto::keys::KeyPair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Proof that a node relayed a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayProof {
    /// The relayer's public key
    pub relayer: PublicKey,
    /// The transaction that was relayed
    pub transaction_id: Hash,
    /// How many hops this relay represents
    pub hop_count: u8,
    /// Timestamp of the relay
    pub timestamp: u64,
    /// Signature by the relayer
    pub signature: Signature,
}

impl RelayProof {
    /// Create a new relay proof
    pub fn new(keypair: &KeyPair, transaction_id: Hash, hop_count: u8) -> Self {
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let signing_data = Self::signing_data(&transaction_id, hop_count, timestamp);
        let signature = keypair.sign(&signing_data);

        RelayProof {
            relayer: keypair.public_key.clone(),
            transaction_id,
            hop_count,
            timestamp,
            signature,
        }
    }

    /// Verify a relay proof
    pub fn verify(&self) -> bool {
        let signing_data = Self::signing_data(&self.transaction_id, self.hop_count, self.timestamp);
        self.relayer.verify(&signing_data, &self.signature)
    }

    fn signing_data(tx_id: &Hash, hop_count: u8, timestamp: u64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(b"RELAY:");
        data.extend_from_slice(tx_id.as_bytes());
        data.push(hop_count);
        data.extend_from_slice(&timestamp.to_le_bytes());
        data
    }
}

/// Tracks relay activity per node for reward calculation
#[derive(Debug, Clone)]
pub struct RelayTracker {
    /// Total relays per node
    relay_counts: HashMap<PublicKey, u64>,
    /// Total relays in the network
    total_relays: u64,
    /// Total rewards distributed
    total_rewards_distributed: u64,
}

impl RelayTracker {
    pub fn new() -> Self {
        RelayTracker {
            relay_counts: HashMap::new(),
            total_relays: 0,
            total_rewards_distributed: 0,
        }
    }

    /// Record a relay and calculate the reward
    pub fn record_relay(&mut self, relayer: &PublicKey) -> u64 {
        let count = self.relay_counts.entry(relayer.clone()).or_insert(0);
        *count += 1;
        let current_count = *count;
        self.total_relays += 1;

        let reward = self.calculate_reward(current_count);

        // Check max supply
        if self.total_rewards_distributed + reward > crate::MAX_SUPPLY {
            return 0; // No more rewards available
        }

        self.total_rewards_distributed += reward;
        reward
    }

    /// Calculate relay reward with diminishing returns
    /// reward = BASE_RELAY_REWARD / (1 + node_relays / RELAY_HALVING_INTERVAL)
    pub fn calculate_reward(&self, node_relay_count: u64) -> u64 {
        let divisor = 1 + node_relay_count / crate::RELAY_HALVING_INTERVAL;
        crate::BASE_RELAY_REWARD / divisor
    }

    /// Get the relay count for a node
    pub fn get_relay_count(&self, relayer: &PublicKey) -> u64 {
        self.relay_counts.get(relayer).copied().unwrap_or(0)
    }

    /// Get total rewards distributed
    pub fn total_rewards(&self) -> u64 {
        self.total_rewards_distributed
    }

    /// Get total relay count
    pub fn total_relays(&self) -> u64 {
        self.total_relays
    }
}

impl Default for RelayTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_relay_proof_creation_and_verification() {
        let kp = KeyPair::generate();
        let tx_id = Hash::digest(b"test_tx");
        let proof = RelayProof::new(&kp, tx_id, 1);

        assert!(proof.verify());
        assert_eq!(proof.hop_count, 1);
        assert_eq!(proof.transaction_id, tx_id);
    }

    #[test]
    fn test_relay_proof_tamper_detection() {
        let kp = KeyPair::generate();
        let tx_id = Hash::digest(b"test_tx");
        let mut proof = RelayProof::new(&kp, tx_id, 1);
        proof.hop_count = 99; // Tamper
        assert!(!proof.verify());
    }

    #[test]
    fn test_relay_tracker_rewards() {
        let mut tracker = RelayTracker::new();
        let kp = KeyPair::generate();

        let reward1 = tracker.record_relay(&kp.public_key);
        assert_eq!(reward1, crate::BASE_RELAY_REWARD); // First relay: full reward

        // After RELAY_HALVING_INTERVAL relays, reward should halve
        for _ in 1..crate::RELAY_HALVING_INTERVAL {
            tracker.record_relay(&kp.public_key);
        }

        let reward_after_halving = tracker.record_relay(&kp.public_key);
        assert!(reward_after_halving < reward1);
    }

    #[test]
    fn test_relay_tracker_fairness() {
        let mut tracker = RelayTracker::new();
        let node1 = KeyPair::generate();
        let node2 = KeyPair::generate();

        // Both nodes get same reward for their first relay
        let r1 = tracker.record_relay(&node1.public_key);
        let r2 = tracker.record_relay(&node2.public_key);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_diminishing_returns() {
        let tracker = RelayTracker::new();

        // Test reward curve
        let r0 = tracker.calculate_reward(0);
        let r1000 = tracker.calculate_reward(1000);
        let r2000 = tracker.calculate_reward(2000);

        assert!(r0 > r1000);
        assert!(r1000 > r2000);

        // First relay: full reward
        assert_eq!(r0, crate::BASE_RELAY_REWARD);
        // After 1000 relays: half reward
        assert_eq!(r1000, crate::BASE_RELAY_REWARD / 2);
    }
}
