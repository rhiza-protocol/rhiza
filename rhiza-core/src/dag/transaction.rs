use crate::crypto::{Hash, PublicKey, Signature};
use crate::crypto::keys::KeyPair;
use serde::{Deserialize, Serialize};

/// The type of transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Transfer RHZ between addresses
    Transfer,
    /// Genesis transaction (creates initial supply via relay rewards)
    Genesis,
    /// Relay reward claim
    RelayReward,
    /// One-time founder allocation at genesis
    FounderAllocation,
}

/// The data payload of a transaction (what gets signed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    /// Type of this transaction
    pub tx_type: TransactionType,
    /// References to 2 parent transactions (DAG structure)
    pub parents: [Hash; 2],
    /// Sender's public key
    pub sender: PublicKey,
    /// Recipient's public key (same as sender for relay rewards)
    pub recipient: PublicKey,
    /// Amount in smallest units (1 RHZ = 10^8)
    pub amount: u64,
    /// Optional fee (0 in current protocol)
    pub fee: u64,
    /// Unix timestamp in milliseconds
    pub timestamp: u64,
    /// Nonce for uniqueness
    pub nonce: u64,
    /// Optional memo/data field
    pub memo: Option<String>,
}

/// A complete transaction with id and signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID (BLAKE3 hash of the signed data)
    pub id: Hash,
    /// The transaction data
    pub data: TransactionData,
    /// Ed25519 signature over the serialized data
    pub signature: Signature,
}

impl TransactionData {
    /// Serialize the transaction data for signing
    pub fn to_signing_bytes(&self) -> Vec<u8> {
        // Use bincode for deterministic serialization
        bincode::serialize(self).expect("serialization should not fail")
    }
}

impl Transaction {
    /// Create and sign a new transaction
    pub fn new(data: TransactionData, keypair: &KeyPair) -> Self {
        let signing_bytes = data.to_signing_bytes();
        let signature = keypair.sign(&signing_bytes);
        let id = Hash::digest(&signing_bytes);

        Transaction {
            id,
            data,
            signature,
        }
    }

    /// Create a genesis transaction
    pub fn genesis(keypair: &KeyPair) -> Self {
        let data = TransactionData {
            tx_type: TransactionType::Genesis,
            parents: [Hash::zero(), Hash::zero()],
            sender: keypair.public_key.clone(),
            recipient: keypair.public_key.clone(),
            amount: 0,
            fee: 0,
            timestamp: 0,
            nonce: 0,
            memo: Some("Rhiza Genesis — The root of true decentralization".to_string()),
        };
        Transaction::new(data, keypair)
    }

    /// Create the founder allocation transaction (one-time genesis allocation)
    pub fn founder_allocation(
        genesis_keypair: &KeyPair,
        founder_pubkey: PublicKey,
        genesis_id: Hash,
    ) -> Self {
        let data = TransactionData {
            tx_type: TransactionType::FounderAllocation,
            parents: [genesis_id, genesis_id],
            sender: genesis_keypair.public_key.clone(),
            recipient: founder_pubkey,
            amount: crate::FOUNDER_ALLOCATION,
            fee: 0,
            timestamp: 0,
            nonce: 1,
            memo: Some("Rhiza Founder Allocation — 5% genesis grant".to_string()),
        };
        Transaction::new(data, genesis_keypair)
    }

    /// Create a transfer transaction
    pub fn transfer(
        sender_keypair: &KeyPair,
        recipient: PublicKey,
        amount: u64,
        parents: [Hash; 2],
        nonce: u64,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let data = TransactionData {
            tx_type: TransactionType::Transfer,
            parents,
            sender: sender_keypair.public_key.clone(),
            recipient,
            amount,
            fee: 0,
            timestamp: now,
            nonce,
            memo: None,
        };
        Transaction::new(data, sender_keypair)
    }

    /// Create a relay reward transaction
    pub fn relay_reward(
        keypair: &KeyPair,
        reward_amount: u64,
        parents: [Hash; 2],
        nonce: u64,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let data = TransactionData {
            tx_type: TransactionType::RelayReward,
            parents,
            sender: keypair.public_key.clone(),
            recipient: keypair.public_key.clone(),
            amount: reward_amount,
            fee: 0,
            timestamp: now,
            nonce,
            memo: None,
        };
        Transaction::new(data, keypair)
    }

    /// Verify the transaction's signature
    pub fn verify_signature(&self) -> bool {
        let signing_bytes = self.data.to_signing_bytes();
        self.data.sender.verify(&signing_bytes, &self.signature)
    }

    /// Verify the transaction ID matches the data
    pub fn verify_id(&self) -> bool {
        let signing_bytes = self.data.to_signing_bytes();
        let expected_id = Hash::digest(&signing_bytes);
        self.id == expected_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_genesis_transaction() {
        let kp = KeyPair::generate();
        let tx = Transaction::genesis(&kp);
        assert_eq!(tx.data.tx_type, TransactionType::Genesis);
        assert!(tx.data.parents[0].is_zero());
        assert!(tx.data.parents[1].is_zero());
        assert!(tx.verify_signature());
        assert!(tx.verify_id());
    }

    #[test]
    fn test_transfer_transaction() {
        let sender = KeyPair::generate();
        let recipient = KeyPair::generate();
        let genesis = Transaction::genesis(&sender);

        let tx = Transaction::transfer(
            &sender,
            recipient.public_key.clone(),
            1_000_000,
            [genesis.id, genesis.id],
            1,
        );

        assert_eq!(tx.data.tx_type, TransactionType::Transfer);
        assert_eq!(tx.data.amount, 1_000_000);
        assert!(tx.verify_signature());
        assert!(tx.verify_id());
    }

    #[test]
    fn test_relay_reward_transaction() {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);

        let tx = Transaction::relay_reward(
            &kp,
            500_000,
            [genesis.id, genesis.id],
            1,
        );

        assert_eq!(tx.data.tx_type, TransactionType::RelayReward);
        assert_eq!(tx.data.sender, tx.data.recipient);
        assert!(tx.verify_signature());
    }

    #[test]
    fn test_transaction_tamper_detection() {
        let sender = KeyPair::generate();
        let recipient = KeyPair::generate();
        let genesis = Transaction::genesis(&sender);

        let mut tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            1_000_000,
            [genesis.id, genesis.id],
            1,
        );

        // Tamper with amount
        tx.data.amount = 999_999_999;
        assert!(!tx.verify_signature());
        assert!(!tx.verify_id());
    }

    #[test]
    fn test_transaction_serialization() {
        let kp = KeyPair::generate();
        let tx = Transaction::genesis(&kp);
        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();
        assert_eq!(tx.id, deserialized.id);
        assert!(deserialized.verify_signature());
    }
}
