use crate::consensus::relay::RelayProof;
use crate::crypto::Hash;
use crate::dag::transaction::Transaction;
use serde::{Deserialize, Serialize};

/// Messages exchanged between peers via gossip protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    /// A new transaction to propagate
    NewTransaction(Transaction),

    /// A relay proof for a transaction
    RelayAnnounce(RelayProof),

    /// Request missing transactions (by ID)
    SyncRequest {
        /// Transaction IDs the requester needs
        missing: Vec<Hash>,
    },

    /// Response to a sync request
    SyncResponse {
        /// The requested transactions
        transactions: Vec<Transaction>,
    },

    /// Announce our tip set (for DAG synchronization)
    TipAnnounce {
        /// Our current tip transaction IDs
        tips: Vec<Hash>,
        /// Our current DAG depth
        depth: u64,
    },

    /// Ping/keepalive
    Ping {
        timestamp: u64,
    },

    /// Pong response
    Pong {
        timestamp: u64,
    },
}

impl GossipMessage {
    /// Serialize to bytes for network transmission
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("gossip message serialization should not fail")
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, GossipError> {
        bincode::deserialize(data).map_err(|e| GossipError::DeserializationError(e.to_string()))
    }

    /// Get a human-readable type name for logging
    pub fn type_name(&self) -> &'static str {
        match self {
            GossipMessage::NewTransaction(_) => "NewTransaction",
            GossipMessage::RelayAnnounce(_) => "RelayAnnounce",
            GossipMessage::SyncRequest { .. } => "SyncRequest",
            GossipMessage::SyncResponse { .. } => "SyncResponse",
            GossipMessage::TipAnnounce { .. } => "TipAnnounce",
            GossipMessage::Ping { .. } => "Ping",
            GossipMessage::Pong { .. } => "Pong",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GossipError {
    #[error("deserialization error: {0}")]
    DeserializationError(String),
    #[error("invalid message")]
    InvalidMessage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_gossip_message_roundtrip() {
        let kp = KeyPair::generate();
        let tx = Transaction::genesis(&kp);
        let msg = GossipMessage::NewTransaction(tx);

        let bytes = msg.to_bytes();
        let decoded = GossipMessage::from_bytes(&bytes).unwrap();

        assert_eq!(msg.type_name(), decoded.type_name());
    }

    #[test]
    fn test_tip_announce() {
        let msg = GossipMessage::TipAnnounce {
            tips: vec![Hash::digest(b"tip1"), Hash::digest(b"tip2")],
            depth: 42,
        };

        let bytes = msg.to_bytes();
        let decoded = GossipMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.type_name(), "TipAnnounce");
    }

    #[test]
    fn test_ping_pong() {
        let ping = GossipMessage::Ping { timestamp: 12345 };
        let bytes = ping.to_bytes();
        let decoded = GossipMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.type_name(), "Ping");
    }
}
