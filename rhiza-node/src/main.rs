use anyhow::Result;
use clap::{Parser, Subcommand};
use rhiza_core::consensus::relay::RelayTracker;
use rhiza_core::crypto::keys::KeyPair;
use rhiza_core::dag::transaction::Transaction;
use rhiza_core::dag::validator::TransactionValidator;
use rhiza_core::dag::vertex::{Dag, DagVertex};
use rhiza_core::network::mesh::MeshConfig;
use rhiza_core::wallet::address::Address;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::info;

mod config;
mod storage;
mod api;

/// Rhiza Node — A truly decentralized currency daemon
#[derive(Parser)]
#[command(name = "rhiza-node", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Data directory
    #[arg(long, default_value = "~/.rhiza")]
    data_dir: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new node
    Init,

    /// Start the node daemon
    Start {
        /// TCP port to listen on
        #[arg(short, long, default_value = "7470")]
        port: u16,
    },

    /// Show node status
    Status,
}

/// The node's state
pub struct NodeState {
    pub dag: Dag,
    pub relay_tracker: RelayTracker,
    pub keypair: KeyPair,
    pub config: MeshConfig,
}

impl NodeState {
    pub fn new(keypair: KeyPair, config: MeshConfig) -> Self {
        NodeState {
            dag: Dag::new(),
            relay_tracker: RelayTracker::new(),
            keypair,
            config,
        }
    }

    /// Initialize the DAG with a genesis transaction if empty
    pub fn initialize_genesis(&mut self) {
        if self.dag.is_empty() {
            let genesis = Transaction::genesis(&self.keypair);
            let genesis_id = genesis.id;
            info!("Creating genesis transaction: {}", genesis_id);
            self.dag
                .insert(DagVertex::new(genesis, 0))
                .expect("genesis insertion should not fail");

            // Create founder allocation (5% of max supply)
            let founder_key_bytes = hex::decode(rhiza_core::FOUNDER_PUBLIC_KEY)
                .expect("invalid founder public key");
            let mut key_arr = [0u8; 32];
            key_arr.copy_from_slice(&founder_key_bytes);
            let founder_pubkey = rhiza_core::crypto::PublicKey::from_bytes(key_arr);

            let founder_tx = Transaction::founder_allocation(
                &self.keypair,
                founder_pubkey,
                genesis_id,
            );
            info!(
                "Creating founder allocation: {} RHZ → founder",
                rhiza_core::FOUNDER_ALLOCATION / rhiza_core::UNITS_PER_RHZ
            );
            self.dag
                .insert(DagVertex::new(founder_tx, 1))
                .expect("founder allocation insertion should not fail");
        }
    }

    /// Process an incoming transaction
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Validate
        TransactionValidator::validate(&tx, &self.dag)
            .map_err(|e| format!("Validation failed: {}", e))?;

        // Calculate depth
        let depth = self.dag.depth() + 1;

        // Insert into DAG
        self.dag
            .insert(DagVertex::new(tx.clone(), depth))
            .map_err(|e| format!("DAG insertion failed: {}", e))?;

        // Record relay
        let reward = self.relay_tracker.record_relay(&self.keypair.public_key);
        if reward > 0 {
            info!("Relay reward: {} units", reward);
        }

        Ok(())
    }

    /// Create and process a transfer transaction
    pub fn send(
        &mut self,
        recipient: rhiza_core::crypto::PublicKey,
        amount: u64,
    ) -> Result<Transaction, String> {
        let parents = self.dag.select_parents();
        let nonce = self.dag.len() as u64;

        let tx = Transaction::transfer(&self.keypair, recipient, amount, parents, nonce);

        // Validate first
        TransactionValidator::validate(&tx, &self.dag)
            .map_err(|e| format!("Validation failed: {}", e))?;

        let depth = self.dag.depth() + 1;
        self.dag
            .insert(DagVertex::new(tx.clone(), depth))
            .map_err(|e| format!("DAG insertion failed: {}", e))?;

        Ok(tx)
    }

    /// Claim a relay reward
    pub fn claim_relay_reward(&mut self) -> Result<Transaction, String> {
        let relay_count = self.relay_tracker.get_relay_count(&self.keypair.public_key);
        let reward = self.relay_tracker.calculate_reward(relay_count);

        if reward == 0 {
            return Err("No reward available".to_string());
        }

        let parents = self.dag.select_parents();
        let nonce = self.dag.len() as u64;

        let tx = Transaction::relay_reward(&self.keypair, reward, parents, nonce);

        let depth = self.dag.depth() + 1;
        self.dag
            .insert(DagVertex::new(tx.clone(), depth))
            .map_err(|e| format!("DAG insertion failed: {}", e))?;

        self.relay_tracker.record_relay(&self.keypair.public_key);

        Ok(tx)
    }

    /// Get this node's balance
    pub fn balance(&self) -> u64 {
        self.dag.get_balance(&self.keypair.public_key)
    }

    /// Get this node's address
    pub fn address(&self) -> Address {
        Address::from_public_key(&self.keypair.public_key)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let data_dir = shellexpand::tilde(&cli.data_dir).to_string();
    let data_path = PathBuf::from(&data_dir);

    match cli.command {
        Commands::Init => {
            info!("🌿 Initializing Rhiza node...");

            // Create data directory
            std::fs::create_dir_all(&data_path)?;

            // Generate keypair
            let keypair = KeyPair::generate();
            let address = Address::from_public_key(&keypair.public_key);

            // Save keystore
            let keystore = rhiza_core::wallet::keystore::KeyStore::from_keypair(&keypair);
            let keystore_path = data_path.join("wallet.json");
            keystore.save(&keystore_path)?;

            println!("🌿 Rhiza Node initialized!");
            println!("📁 Data directory: {}", data_dir);
            println!("🔑 Address: {}", address);
            println!("⚠️  Keep your wallet.json safe — it contains your private key!");

            Ok(())
        }

        Commands::Start { port } => {
            info!("🌿 Starting Rhiza node on port {}...", port);

            // Load keypair
            let keystore_path = data_path.join("wallet.json");
            if !keystore_path.exists() {
                anyhow::bail!("Node not initialized. Run 'rhiza-node init' first.");
            }

            let keystore = rhiza_core::wallet::keystore::KeyStore::load(&keystore_path)?;
            let keypair = keystore.to_keypair()?;
            let address = Address::from_public_key(&keypair.public_key);

            let config = MeshConfig::local_test(port);
            let mut state = NodeState::new(keypair, config);
            state.initialize_genesis();

            println!("🌿 Rhiza Node running!");
            println!("🔑 Address: {}", address);
            println!("📊 DAG size: {} transactions", state.dag.len());
            println!("🌐 Listening on port {}", port);
            println!("Press Ctrl+C to stop");

            // Start the REST API server
            let shared_state = Arc::new(Mutex::new(state));
            let api_handle = tokio::spawn(api::run_api_server(shared_state.clone(), port + 1));

            info!("REST API available at http://127.0.0.1:{}", port + 1);

            // Wait for shutdown signal
            tokio::signal::ctrl_c().await?;
            info!("Shutting down...");

            Ok(())
        }

        Commands::Status => {
            let keystore_path = data_path.join("wallet.json");
            if !keystore_path.exists() {
                println!("❌ Node not initialized. Run 'rhiza-node init' first.");
                return Ok(());
            }

            let keystore = rhiza_core::wallet::keystore::KeyStore::load(&keystore_path)?;
            let keypair = keystore.to_keypair()?;
            let address = Address::from_public_key(&keypair.public_key);

            println!("🌿 Rhiza Node Status");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🔑 Address: {}", address);
            println!("📁 Data: {}", data_dir);

            Ok(())
        }
    }
}
