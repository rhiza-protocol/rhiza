use crate::crypto::{Hash, PublicKey};
use bech32::{Bech32m, Hrp};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A Rhiza address in bech32m format (e.g., rhz1...)
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    /// Create an address from a public key
    pub fn from_public_key(pubkey: &PublicKey) -> Self {
        // Hash the public key for shorter address
        let hash = Hash::digest(pubkey.as_bytes());
        let hash_bytes = &hash.as_bytes()[..20]; // Take first 20 bytes

        let hrp = Hrp::parse(crate::ADDRESS_HRP).expect("valid HRP");
        let encoded = bech32::encode::<Bech32m>(hrp, hash_bytes)
            .expect("valid bech32m encoding");

        Address(encoded)
    }

    /// Parse an address from string
    pub fn from_str(s: &str) -> Result<Self, AddressError> {
        let hrp = Hrp::parse(crate::ADDRESS_HRP).map_err(|_| AddressError::InvalidHrp)?;
        let (decoded_hrp, data) =
            bech32::decode(s).map_err(|_| AddressError::InvalidEncoding)?;

        if decoded_hrp != hrp {
            return Err(AddressError::InvalidHrp);
        }

        if data.len() != 20 {
            return Err(AddressError::InvalidLength);
        }

        Ok(Address(s.to_string()))
    }

    /// Get the raw string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({})", &self.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("invalid address encoding")]
    InvalidEncoding,
    #[error("invalid human-readable prefix (expected 'rhz')")]
    InvalidHrp,
    #[error("invalid address data length")]
    InvalidLength,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;

    #[test]
    fn test_address_from_public_key() {
        let kp = KeyPair::generate();
        let addr = Address::from_public_key(&kp.public_key);
        assert!(addr.as_str().starts_with("rhz1"));
    }

    #[test]
    fn test_address_deterministic() {
        let kp = KeyPair::generate();
        let addr1 = Address::from_public_key(&kp.public_key);
        let addr2 = Address::from_public_key(&kp.public_key);
        assert_eq!(addr1, addr2);
    }

    #[test]
    fn test_address_parse_roundtrip() {
        let kp = KeyPair::generate();
        let addr = Address::from_public_key(&kp.public_key);
        let parsed = Address::from_str(addr.as_str()).unwrap();
        assert_eq!(addr, parsed);
    }

    #[test]
    fn test_address_invalid() {
        assert!(Address::from_str("btc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4").is_err());
        assert!(Address::from_str("invalid").is_err());
    }
}
