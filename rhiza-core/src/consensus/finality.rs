use crate::crypto::Hash;
use crate::dag::vertex::Dag;

/// Finality checker — determines if a transaction is irreversibly confirmed
pub struct FinalityChecker;

impl FinalityChecker {
    /// Check if a specific transaction has reached finality
    pub fn is_final(dag: &Dag, tx_id: &Hash) -> bool {
        match dag.get(tx_id) {
            Some(vertex) => vertex.cumulative_weight >= crate::FINALITY_THRESHOLD,
            None => false,
        }
    }

    /// Get finality status for a transaction
    pub fn finality_status(dag: &Dag, tx_id: &Hash) -> FinalityStatus {
        match dag.get(tx_id) {
            None => FinalityStatus::Unknown,
            Some(vertex) => {
                if vertex.cumulative_weight >= crate::FINALITY_THRESHOLD {
                    FinalityStatus::Final
                } else if vertex.cumulative_weight > 1 {
                    FinalityStatus::Confirming {
                        weight: vertex.cumulative_weight,
                        needed: crate::FINALITY_THRESHOLD,
                    }
                } else {
                    FinalityStatus::Pending
                }
            }
        }
    }

    /// Get all final transactions in the DAG
    pub fn get_final_transactions(dag: &Dag) -> Vec<Hash> {
        dag.transaction_ids()
            .into_iter()
            .filter(|id| Self::is_final(dag, id))
            .collect()
    }
}

/// Status of a transaction's finality
#[derive(Debug, Clone, PartialEq)]
pub enum FinalityStatus {
    /// Transaction not found in DAG
    Unknown,
    /// Transaction is in DAG but has no confirmations yet
    Pending,
    /// Transaction is being confirmed
    Confirming { weight: u64, needed: u64 },
    /// Transaction has reached finality
    Final,
}

impl std::fmt::Display for FinalityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinalityStatus::Unknown => write!(f, "❓ Unknown"),
            FinalityStatus::Pending => write!(f, "⏳ Pending"),
            FinalityStatus::Confirming { weight, needed } => {
                write!(f, "🔄 Confirming ({}/{})", weight, needed)
            }
            FinalityStatus::Final => write!(f, "✅ Final"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;
    use crate::dag::transaction::Transaction;
    use crate::dag::vertex::DagVertex;

    #[test]
    fn test_finality_progression() {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let genesis_id = genesis.id;
        let mut dag = Dag::new();
        dag.insert(DagVertex::new(genesis, 0)).unwrap();

        // Genesis starts as pending
        assert_eq!(
            FinalityChecker::finality_status(&dag, &genesis_id),
            FinalityStatus::Pending
        );

        // Add transactions that reference genesis
        let mut last_ids = [genesis_id, genesis_id];
        for i in 1..=crate::FINALITY_THRESHOLD {
            let tx = Transaction::relay_reward(&kp, 100, last_ids, i);
            let tx_id = tx.id;
            dag.insert(DagVertex::new(tx, i)).unwrap();
            last_ids = [tx_id, tx_id];
        }

        // Genesis should now be final
        assert!(FinalityChecker::is_final(&dag, &genesis_id));
        assert_eq!(
            FinalityChecker::finality_status(&dag, &genesis_id),
            FinalityStatus::Final
        );
    }
}
