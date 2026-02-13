use crate::crypto::keys::KeyPair;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Encrypted keystore for wallet management
#[derive(Serialize, Deserialize)]
pub struct KeyStore {
    /// Hex-encoded encrypted secret key (for simplicity, using plain encoding in prototype)
    secret_key_hex: String,
    /// The public key hex for identification
    public_key_hex: String,
    /// Creation timestamp
    created_at: String,
}

impl KeyStore {
    /// Create a new keystore from a keypair
    pub fn from_keypair(keypair: &KeyPair) -> Self {
        KeyStore {
            secret_key_hex: hex::encode(keypair.secret_bytes()),
            public_key_hex: keypair.public_key.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Save the keystore to a file
    pub fn save(&self, path: &Path) -> Result<(), KeyStoreError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(KeyStoreError::Io)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(KeyStoreError::Serialize)?;
        fs::write(path, json).map_err(KeyStoreError::Io)?;
        Ok(())
    }

    /// Load a keystore from a file
    pub fn load(path: &Path) -> Result<Self, KeyStoreError> {
        let data = fs::read_to_string(path).map_err(KeyStoreError::Io)?;
        let ks: KeyStore = serde_json::from_str(&data).map_err(KeyStoreError::Deserialize)?;
        Ok(ks)
    }

    /// Recover the keypair from stored data
    pub fn to_keypair(&self) -> Result<KeyPair, KeyStoreError> {
        let bytes = hex::decode(&self.secret_key_hex)
            .map_err(|_| KeyStoreError::InvalidKey)?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| KeyStoreError::InvalidKey)?;
        Ok(KeyPair::from_secret_bytes(&arr))
    }

    /// Get the public key hex
    pub fn public_key_hex(&self) -> &str {
        &self.public_key_hex
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeyStoreError {
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("serialization error: {0}")]
    Serialize(serde_json::Error),
    #[error("deserialization error: {0}")]
    Deserialize(serde_json::Error),
    #[error("invalid key data")]
    InvalidKey,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_keystore_save_load() {
        let kp = KeyPair::generate();
        let ks = KeyStore::from_keypair(&kp);

        let dir = tempdir().unwrap();
        let path = dir.path().join("wallet.json");

        ks.save(&path).unwrap();
        let loaded = KeyStore::load(&path).unwrap();
        let recovered = loaded.to_keypair().unwrap();

        assert_eq!(kp.public_key, recovered.public_key);
    }
}
