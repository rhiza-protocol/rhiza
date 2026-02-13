use anyhow::Result;
use clap::{Parser, Subcommand};
use rhiza_core::crypto::keys::KeyPair;
use rhiza_core::wallet::address::Address;
use rhiza_core::wallet::keystore::KeyStore;
use std::path::PathBuf;

/// Rhiza CLI — Wallet and tools for the Rhiza decentralized currency
#[derive(Parser)]
#[command(
    name = "rhiza",
    version,
    about = "🌿 Rhiza — The root of true decentralization",
    long_about = "Rhiza is a truly decentralized currency using DAG structure, Proof of Relay consensus, and mesh networking. No mining, no staking — just fair participation."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Wallet directory
    #[arg(long, default_value = "~/.rhiza")]
    wallet_dir: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Wallet management
    Wallet {
        #[command(subcommand)]
        action: WalletCommands,
    },

    /// Show network information
    Info,

    /// Show protocol constants
    Protocol,
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Create a new wallet
    Create,

    /// Show wallet address and balance
    Show,

    /// Show the public key
    Pubkey,

    /// Export wallet (display secret key — be careful!)
    Export,
}

fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs_next::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let wallet_dir = expand_path(&cli.wallet_dir);
    let wallet_path = wallet_dir.join("wallet.json");

    match cli.command {
        Commands::Wallet { action } => match action {
            WalletCommands::Create => {
                if wallet_path.exists() {
                    println!("⚠️  Wallet already exists at {}", wallet_path.display());
                    println!("   Delete it first if you want to create a new one.");
                    return Ok(());
                }

                let keypair = KeyPair::generate();
                let address = Address::from_public_key(&keypair.public_key);
                let keystore = KeyStore::from_keypair(&keypair);

                std::fs::create_dir_all(&wallet_dir)?;
                keystore.save(&wallet_path)?;

                println!();
                println!("  🌿 Rhiza Wallet Created!");
                println!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("  📍 Address:    {}", address);
                println!("  🔑 Public Key: {}", keypair.public_key);
                println!("  📁 Saved to:   {}", wallet_path.display());
                println!();
                println!("  ⚠️  IMPORTANT: Back up your wallet.json file!");
                println!("     Losing it means losing access to your RHZ forever.");
                println!();

                Ok(())
            }

            WalletCommands::Show => {
                let keystore = load_wallet(&wallet_path)?;
                let keypair = keystore.to_keypair()?;
                let address = Address::from_public_key(&keypair.public_key);

                // Check if this wallet is the founder
                let is_founder = format!("{}", keypair.public_key) == rhiza_core::FOUNDER_PUBLIC_KEY;
                let founder_rhz = rhiza_core::FOUNDER_ALLOCATION / rhiza_core::UNITS_PER_RHZ;

                println!();
                println!("  🌿 Rhiza Wallet");
                println!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("  📍 Address:    {}", address);
                println!("  🔑 Public Key: {}", keypair.public_key);
                if is_founder {
                    println!();
                    println!("  👑 Status:     FOUNDER");
                    println!("  💰 Allocation: {} RHZ (5% genesis grant)", founder_rhz);
                    println!("  📊 Balance:    {} RHZ", founder_rhz);
                }
                println!();

                Ok(())
            }

            WalletCommands::Pubkey => {
                let keystore = load_wallet(&wallet_path)?;
                let keypair = keystore.to_keypair()?;
                println!("{}", keypair.public_key);
                Ok(())
            }

            WalletCommands::Export => {
                let keystore = load_wallet(&wallet_path)?;
                let keypair = keystore.to_keypair()?;

                println!();
                println!("  ⚠️  WARNING: Never share your secret key!");
                println!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("  🔐 Secret Key: {}", hex::encode(keypair.secret_bytes()));
                println!("  🔑 Public Key: {}", keypair.public_key);
                println!();

                Ok(())
            }
        },

        Commands::Info => {
            println!();
            println!("  🌿 Rhiza Network Information");
            println!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("  Protocol:     Rhiza v0.1.0");
            println!("  Consensus:    Proof of Relay (PoR)");
            println!("  Structure:    Directed Acyclic Graph (DAG)");
            println!("  Hash:         BLAKE3");
            println!("  Signatures:   Ed25519");
            println!("  Addresses:    Bech32m (rhz1...)");
            println!("  Default Port: 7470");
            println!();

            Ok(())
        }

        Commands::Protocol => {
            println!();
            println!("  🌿 Rhiza Protocol Constants");
            println!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!(
                "  Max Supply:           {:>15} RHZ",
                rhiza_core::MAX_SUPPLY / rhiza_core::UNITS_PER_RHZ
            );
            println!(
                "  Units per RHZ:        {:>15}",
                rhiza_core::UNITS_PER_RHZ
            );
            println!(
                "  Base Relay Reward:    {:>15} units ({:.4} RHZ)",
                rhiza_core::BASE_RELAY_REWARD,
                rhiza_core::BASE_RELAY_REWARD as f64 / rhiza_core::UNITS_PER_RHZ as f64
            );
            println!(
                "  Halving Interval:     {:>15} relays",
                rhiza_core::RELAY_HALVING_INTERVAL
            );
            println!(
                "  Finality Threshold:   {:>15} weight",
                rhiza_core::FINALITY_THRESHOLD
            );
            println!(
                "  Parent References:    {:>15}",
                rhiza_core::PARENT_COUNT
            );
            println!(
                "  Founder Allocation:   {:>15} RHZ (5%)",
                rhiza_core::FOUNDER_ALLOCATION / rhiza_core::UNITS_PER_RHZ
            );
            println!();

            Ok(())
        }
    }
}

fn load_wallet(path: &PathBuf) -> Result<KeyStore> {
    if !path.exists() {
        anyhow::bail!(
            "No wallet found. Create one with: rhiza wallet create"
        );
    }
    Ok(KeyStore::load(path)?)
}
