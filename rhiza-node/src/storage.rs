use rhiza_core::crypto::Hash;
use rhiza_core::dag::transaction::Transaction;
use sled::Db;
use std::path::Path;

/// Persistent storage for DAG data using sled embedded database
pub struct Storage {
    db: Db,
}

impl Storage {
    /// Open or create a storage database
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Storage { db })
    }

    /// Store a transaction
    pub fn put_transaction(&self, tx: &Transaction) -> anyhow::Result<()> {
        let key = tx.id.as_bytes();
        let value = bincode::serialize(tx)?;
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Get a transaction by ID
    pub fn get_transaction(&self, id: &Hash) -> anyhow::Result<Option<Transaction>> {
        match self.db.get(id.as_bytes())? {
            Some(data) => {
                let tx: Transaction = bincode::deserialize(&data)?;
                Ok(Some(tx))
            }
            None => Ok(None),
        }
    }

    /// Get all stored transactions
    pub fn get_all_transactions(&self) -> anyhow::Result<Vec<Transaction>> {
        let mut txs = Vec::new();
        for result in self.db.iter() {
            let (_, value) = result?;
            let tx: Transaction = bincode::deserialize(&value)?;
            txs.push(tx);
        }
        Ok(txs)
    }

    /// Number of stored transactions
    pub fn count(&self) -> usize {
        self.db.len()
    }
}
