use serde::{Deserialize, Serialize};
use std::fmt;

/// A BLAKE3 hash value (32 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(#[serde(with = "hash_serde")] pub(crate) [u8; 32]);

impl Hash {
    /// Hash arbitrary data
    pub fn digest(data: &[u8]) -> Self {
        let h = blake3::hash(data);
        Hash(*h.as_bytes())
    }

    /// Hash multiple pieces of data together
    pub fn digest_multi(parts: &[&[u8]]) -> Self {
        let mut hasher = blake3::Hasher::new();
        for part in parts {
            hasher.update(part);
        }
        Hash(*hasher.finalize().as_bytes())
    }

    /// The zero hash (used as genesis reference)
    pub fn zero() -> Self {
        Hash([0u8; 32])
    }

    /// Check if this is the zero hash
    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 32]
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Hash(bytes)
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({}..)", hex::encode(&self.0[..8]))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

mod hash_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("invalid hash length"))?;
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_digest() {
        let h = Hash::digest(b"hello rhiza");
        assert_ne!(h, Hash::zero());
    }

    #[test]
    fn test_hash_deterministic() {
        let h1 = Hash::digest(b"test");
        let h2 = Hash::digest(b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_different_inputs() {
        let h1 = Hash::digest(b"hello");
        let h2 = Hash::digest(b"world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_multi() {
        let h1 = Hash::digest_multi(&[b"hello", b"world"]);
        let h2 = Hash::digest(b"helloworld");
        // digest_multi uses streaming, should produce same result
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_zero_hash() {
        let z = Hash::zero();
        assert!(z.is_zero());
        let h = Hash::digest(b"test");
        assert!(!h.is_zero());
    }

    #[test]
    fn test_hash_serialization() {
        let h = Hash::digest(b"test");
        let json = serde_json::to_string(&h).unwrap();
        let deserialized: Hash = serde_json::from_str(&json).unwrap();
        assert_eq!(h, deserialized);
    }
}
