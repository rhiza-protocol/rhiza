pub mod crypto;
pub mod dag;
pub mod consensus;
pub mod network;
pub mod wallet;

/// The human-readable prefix for Rhiza addresses
pub const ADDRESS_HRP: &str = "rhz";

/// Smallest unit: 1 RHZ = 10^8 rhiza (like satoshis)
pub const UNITS_PER_RHZ: u64 = 100_000_000;

/// Maximum supply: 21,000,000 RHZ
pub const MAX_SUPPLY: u64 = 21_000_000 * UNITS_PER_RHZ;

/// Number of parent references each transaction must have
pub const PARENT_COUNT: usize = 2;

/// Minimum cumulative weight for finality
pub const FINALITY_THRESHOLD: u64 = 10;

/// Base relay reward in smallest units (0.01 RHZ)
pub const BASE_RELAY_REWARD: u64 = 1_000_000;

/// Relay count at which reward halves
pub const RELAY_HALVING_INTERVAL: u64 = 1_000;

/// Founder allocation: 5% of max supply (1,050,000 RHZ)
/// This is a one-time genesis allocation to the protocol creator
pub const FOUNDER_ALLOCATION: u64 = MAX_SUPPLY / 20;

/// Founder's public key (Ed25519, hex-encoded)
/// Address: rhz1hh8kfkldmn37t35wqqaz9t9rtrhnk4e9qlkz5z
pub const FOUNDER_PUBLIC_KEY: &str = "cd3f2d882dd11f282e13f641b6aa751a3d46b3ff5a9efbccebea9a0131c0dfdd";
