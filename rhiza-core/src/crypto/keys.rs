use ed25519_dalek::{
    SigningKey, VerifyingKey, Signer, Verifier,
    Signature as DalekSignature,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Wrapper around Ed25519 public key
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey(#[serde(with = "pub_key_serde")] pub(crate) [u8; 32]);

/// Wrapper around Ed25519 secret key bytes
#[derive(Clone, Serialize, Deserialize)]
pub struct SecretKey(#[serde(with = "hex_serde")] pub(crate) [u8; 32]);

/// Wrapper around Ed25519 signature
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(#[serde(with = "hex_serde_64")] pub(crate) [u8; 64]);

/// A keypair consisting of a secret key and its corresponding public key
#[derive(Clone)]
pub struct KeyPair {
    signing_key: SigningKey,
    pub public_key: PublicKey,
}

impl KeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        KeyPair {
            signing_key,
            public_key: PublicKey(verifying_key.to_bytes()),
        }
    }

    /// Restore a keypair from secret key bytes
    pub fn from_secret_bytes(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();
        KeyPair {
            signing_key,
            public_key: PublicKey(verifying_key.to_bytes()),
        }
    }

    /// Get the secret key bytes
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        let sig = self.signing_key.sign(message);
        Signature(sig.to_bytes())
    }
}

impl PublicKey {
    /// Verify a signature against this public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        let Ok(verifying_key) = VerifyingKey::from_bytes(&self.0) else {
            return false;
        };
        let sig = DalekSignature::from_bytes(&signature.0);
        verifying_key.verify(message, &sig).is_ok()
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        PublicKey(bytes)
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({})", hex::encode(&self.0[..8]))
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sig({}..)", hex::encode(&self.0[..8]))
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretKey([REDACTED])")
    }
}

// --- Serde helpers ---

mod pub_key_serde {
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
            .map_err(|_| serde::de::Error::custom("invalid public key length"))?;
        Ok(arr)
    }
}

mod hex_serde {
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
            .map_err(|_| serde::de::Error::custom("invalid length"))?;
        Ok(arr)
    }
}

mod hex_serde_64 {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        let arr: [u8; 64] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("invalid signature length"))?;
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate();
        assert_ne!(kp.public_key.as_bytes(), &[0u8; 32]);
    }

    #[test]
    fn test_sign_and_verify() {
        let kp = KeyPair::generate();
        let message = b"hello rhiza";
        let sig = kp.sign(message);
        assert!(kp.public_key.verify(message, &sig));
    }

    #[test]
    fn test_verify_wrong_message() {
        let kp = KeyPair::generate();
        let sig = kp.sign(b"hello");
        assert!(!kp.public_key.verify(b"goodbye", &sig));
    }

    #[test]
    fn test_verify_wrong_key() {
        let kp1 = KeyPair::generate();
        let kp2 = KeyPair::generate();
        let sig = kp1.sign(b"hello");
        assert!(!kp2.public_key.verify(b"hello", &sig));
    }

    #[test]
    fn test_keypair_restore() {
        let kp = KeyPair::generate();
        let secret = kp.secret_bytes();
        let restored = KeyPair::from_secret_bytes(&secret);
        assert_eq!(kp.public_key, restored.public_key);
    }

    #[test]
    fn test_pubkey_serialization() {
        let kp = KeyPair::generate();
        let json = serde_json::to_string(&kp.public_key).unwrap();
        let deserialized: PublicKey = serde_json::from_str(&json).unwrap();
        assert_eq!(kp.public_key, deserialized);
    }
}
