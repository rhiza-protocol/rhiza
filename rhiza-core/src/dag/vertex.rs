use crate::crypto::Hash;
use crate::dag::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A vertex in the DAG — wraps a transaction with DAG metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagVertex {
    /// The transaction at this vertex
    pub transaction: Transaction,
    /// Cumulative weight (own weight + weight of all approvers)
    pub cumulative_weight: u64,
    /// Direct weight (1 for valid tx)
    pub own_weight: u64,
    /// Whether this transaction has reached finality
    pub is_final: bool,
    /// Depth in the DAG (distance from genesis)
    pub depth: u64,
}

impl DagVertex {
    /// Create a new vertex from a transaction
    pub fn new(transaction: Transaction, depth: u64) -> Self {
        DagVertex {
            transaction,
            cumulative_weight: 1, // Own weight
            own_weight: 1,
            is_final: false,
            depth,
        }
    }

    /// Get the transaction ID
    pub fn id(&self) -> Hash {
        self.transaction.id
    }

    /// Get the parent references
    pub fn parents(&self) -> &[Hash; 2] {
        &self.transaction.data.parents
    }
}

/// The DAG structure — stores all vertices and their relationships
#[derive(Debug, Clone)]
pub struct Dag {
    /// All vertices indexed by transaction ID
    vertices: HashMap<Hash, DagVertex>,
    /// Mapping from vertex ID to IDs of vertices that reference it (children/approvers)
    children: HashMap<Hash, Vec<Hash>>,
    /// Tips: vertices with no children (frontier of the DAG)
    tips: Vec<Hash>,
    /// The genesis transaction ID
    pub genesis_id: Option<Hash>,
}

impl Dag {
    /// Create a new empty DAG
    pub fn new() -> Self {
        Dag {
            vertices: HashMap::new(),
            children: HashMap::new(),
            tips: Vec::new(),
            genesis_id: None,
        }
    }

    /// Insert a vertex into the DAG
    pub fn insert(&mut self, vertex: DagVertex) -> Result<(), DagError> {
        let id = vertex.id();

        // Check for duplicates
        if self.vertices.contains_key(&id) {
            return Err(DagError::DuplicateTransaction);
        }

        // For non-genesis transactions, verify parents exist
        if !vertex.parents()[0].is_zero() {
            for parent in vertex.parents() {
                if !self.vertices.contains_key(parent) {
                    return Err(DagError::MissingParent(*parent));
                }
                // Register this vertex as a child of its parent
                self.children
                    .entry(*parent)
                    .or_default()
                    .push(id);

                // Parent is no longer a tip
                self.tips.retain(|t| t != parent);
            }
        }

        // Track genesis
        if vertex.transaction.data.parents[0].is_zero() && self.genesis_id.is_none() {
            self.genesis_id = Some(id);
        }

        // New vertex is a tip
        self.tips.push(id);

        self.vertices.insert(id, vertex);

        // Update cumulative weights
        self.update_weights(id);

        Ok(())
    }

    /// Get a vertex by ID
    pub fn get(&self, id: &Hash) -> Option<&DagVertex> {
        self.vertices.get(id)
    }

    /// Get current tips (for selecting parents for new transactions)
    pub fn tips(&self) -> &[Hash] {
        &self.tips
    }

    /// Select 2 tips for a new transaction's parents
    pub fn select_parents(&self) -> [Hash; 2] {
        match self.tips.len() {
            0 => [Hash::zero(), Hash::zero()],
            1 => [self.tips[0], self.tips[0]],
            _ => {
                // Select 2 most recent tips (by depth)
                let mut sorted_tips: Vec<_> = self.tips.iter()
                    .filter_map(|t| self.vertices.get(t).map(|v| (t, v.depth)))
                    .collect();
                sorted_tips.sort_by(|a, b| b.1.cmp(&a.1));

                [*sorted_tips[0].0, *sorted_tips[1].0]
            }
        }
    }

    /// Number of vertices in the DAG
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Check if DAG is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Get the current depth (max depth of any vertex)
    pub fn depth(&self) -> u64 {
        self.vertices.values().map(|v| v.depth).max().unwrap_or(0)
    }

    /// Get all transaction IDs
    pub fn transaction_ids(&self) -> Vec<Hash> {
        self.vertices.keys().copied().collect()
    }

    /// Update cumulative weights after inserting a vertex
    fn update_weights(&mut self, new_vertex_id: Hash) {
        // Walk back through parents and increment their cumulative weight
        let mut stack = vec![new_vertex_id];
        let mut visited = std::collections::HashSet::new();

        while let Some(id) = stack.pop() {
            if !visited.insert(id) {
                continue;
            }

            if let Some(vertex) = self.vertices.get(&id) {
                let parents = *vertex.parents();
                for parent in &parents {
                    if !parent.is_zero() {
                        if let Some(parent_vertex) = self.vertices.get_mut(parent) {
                            parent_vertex.cumulative_weight += 1;
                            // Check finality
                            if parent_vertex.cumulative_weight >= crate::FINALITY_THRESHOLD {
                                parent_vertex.is_final = true;
                            }
                        }
                        stack.push(*parent);
                    }
                }
            }
        }
    }

    /// Get the balance of a public key by traversing the DAG
    pub fn get_balance(&self, pubkey: &crate::crypto::PublicKey) -> u64 {
        let mut balance: i128 = 0;

        for vertex in self.vertices.values() {
            let tx = &vertex.transaction;

            // Add received amounts
            if tx.data.recipient == *pubkey {
                balance += tx.data.amount as i128;
            }

            // Subtract sent amounts (only for transfers, not self-payments)
            if tx.data.sender == *pubkey
                && tx.data.recipient != *pubkey
            {
                balance -= tx.data.amount as i128;
                balance -= tx.data.fee as i128;
            }
        }

        balance.max(0) as u64
    }
}

impl Default for Dag {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DagError {
    #[error("duplicate transaction")]
    DuplicateTransaction,
    #[error("missing parent transaction: {0}")]
    MissingParent(Hash),
    #[error("invalid transaction")]
    InvalidTransaction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;
    use crate::dag::transaction::Transaction;

    fn setup_dag_with_genesis() -> (Dag, KeyPair, Hash) {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let genesis_id = genesis.id;
        let mut dag = Dag::new();
        dag.insert(DagVertex::new(genesis, 0)).unwrap();
        (dag, kp, genesis_id)
    }

    #[test]
    fn test_insert_genesis() {
        let (dag, _, genesis_id) = setup_dag_with_genesis();
        assert_eq!(dag.len(), 1);
        assert_eq!(dag.genesis_id, Some(genesis_id));
    }

    #[test]
    fn test_insert_transaction() {
        let (mut dag, sender, genesis_id) = setup_dag_with_genesis();
        let recipient = KeyPair::generate();

        let tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            1_000_000,
            [genesis_id, genesis_id],
            1,
        );
        dag.insert(DagVertex::new(tx, 1)).unwrap();

        assert_eq!(dag.len(), 2);
        assert_eq!(dag.depth(), 1);
    }

    #[test]
    fn test_cumulative_weight() {
        let (mut dag, sender, genesis_id) = setup_dag_with_genesis();
        let recipient = KeyPair::generate();

        // Add a transaction referencing genesis
        let tx1 = Transaction::transfer(
            &sender,
            recipient.public_key.clone(),
            100,
            [genesis_id, genesis_id],
            1,
        );
        dag.insert(DagVertex::new(tx1, 1)).unwrap();

        // Genesis should now have cumulative_weight 2 (1 own + 1 from child)
        let genesis = dag.get(&genesis_id).unwrap();
        assert!(genesis.cumulative_weight > 1);
    }

    #[test]
    fn test_tips() {
        let (mut dag, sender, genesis_id) = setup_dag_with_genesis();
        assert_eq!(dag.tips().len(), 1);

        let recipient = KeyPair::generate();
        let tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            100,
            [genesis_id, genesis_id],
            1,
        );
        let tx_id = tx.id;
        dag.insert(DagVertex::new(tx, 1)).unwrap();

        // Genesis should no longer be a tip
        assert_eq!(dag.tips().len(), 1);
        assert_eq!(dag.tips()[0], tx_id);
    }

    #[test]
    fn test_duplicate_prevention() {
        let (mut dag, kp, _) = setup_dag_with_genesis();
        let genesis2 = Transaction::genesis(&kp);
        let vertex = DagVertex::new(genesis2, 0);
        // Same keypair genesis produces same content, hence same id
        let result = dag.insert(vertex);
        assert!(result.is_err());
    }

    #[test]
    fn test_select_parents() {
        let (dag, _, genesis_id) = setup_dag_with_genesis();
        let parents = dag.select_parents();
        assert_eq!(parents, [genesis_id, genesis_id]);
    }
}
