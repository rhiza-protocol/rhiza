use crate::crypto::PublicKey;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;

/// Unique identifier for a peer in the network
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId {
    /// The peer's public key
    pub public_key: PublicKey,
}

/// Information about a connected peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer identifier
    pub id: PeerId,
    /// Network address (optional — mesh peers might not have an IP)
    pub address: Option<SocketAddr>,
    /// Protocol version
    pub protocol_version: u32,
    /// Node software version
    pub agent_version: String,
    /// When we first connected
    pub connected_since: u64,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Number of messages relayed from this peer
    pub messages_relayed: u64,
}

impl PeerId {
    pub fn new(public_key: PublicKey) -> Self {
        PeerId { public_key }
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Peer({})", &self.public_key.to_string()[..16])
    }
}

/// Current protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Agent version string
pub const AGENT_VERSION: &str = "rhiza/0.1.0";
