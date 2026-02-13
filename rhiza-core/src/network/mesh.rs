use serde::{Deserialize, Serialize};

/// Mesh transport layer abstraction
/// Supports multiple transport types for true censorship resistance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportType {
    /// Standard TCP/IP over internet
    Tcp,
    /// WiFi Direct (peer-to-peer WiFi)
    WifiDirect,
    /// Bluetooth Low Energy
    Bluetooth,
    /// LoRa (Long Range radio)
    LoRa,
    /// mDNS (Local network discovery)
    Mdns,
}

/// Configuration for mesh networking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshConfig {
    /// Enabled transport types
    pub transports: Vec<TransportType>,
    /// Maximum number of peers
    pub max_peers: usize,
    /// TCP listen port
    pub tcp_port: u16,
    /// Whether to enable local network discovery
    pub enable_mdns: bool,
    /// Bootstrap peers (TCP addresses)
    pub bootstrap_peers: Vec<String>,
}

impl Default for MeshConfig {
    fn default() -> Self {
        MeshConfig {
            transports: vec![TransportType::Tcp, TransportType::Mdns],
            max_peers: 50,
            tcp_port: 7470, // R=7, H=4, Z=7, 0
            enable_mdns: true,
            bootstrap_peers: Vec::new(),
        }
    }
}

impl MeshConfig {
    /// Create a config for local testing
    pub fn local_test(port: u16) -> Self {
        MeshConfig {
            transports: vec![TransportType::Tcp, TransportType::Mdns],
            max_peers: 10,
            tcp_port: port,
            enable_mdns: true,
            bootstrap_peers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MeshConfig::default();
        assert_eq!(config.tcp_port, 7470);
        assert!(config.enable_mdns);
        assert_eq!(config.max_peers, 50);
    }

    #[test]
    fn test_local_test_config() {
        let config = MeshConfig::local_test(9999);
        assert_eq!(config.tcp_port, 9999);
    }
}
