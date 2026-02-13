use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node display name
    pub name: String,
    /// TCP port for P2P connections
    pub p2p_port: u16,
    /// REST API port
    pub api_port: u16,
    /// Data directory path
    pub data_dir: PathBuf,
    /// Maximum peer connections
    pub max_peers: usize,
    /// Enable mDNS local discovery
    pub enable_mdns: bool,
    /// Bootstrap peer addresses
    pub bootstrap_peers: Vec<String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            name: "rhiza-node".to_string(),
            p2p_port: 7470,
            api_port: 7471,
            data_dir: PathBuf::from("~/.rhiza"),
            max_peers: 50,
            enable_mdns: true,
            bootstrap_peers: Vec::new(),
        }
    }
}

impl NodeConfig {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let config: NodeConfig = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
