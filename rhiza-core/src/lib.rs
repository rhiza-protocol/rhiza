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
