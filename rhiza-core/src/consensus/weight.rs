use crate::crypto::Hash;
use crate::dag::vertex::Dag;

/// Cumulative weight calculation for DAG vertices
///
/// Weight determines how "confirmed" a transaction is.
/// A higher cumulative weight means more transactions have approved it (directly or indirectly).
pub struct WeightCalculator;

impl WeightCalculator {
    /// Recalculate all cumulative weights in the DAG from scratch
    /// This is used for verification; normally weights are updated incrementally
    pub fn calculate_all_weights(dag: &Dag) -> std::collections::HashMap<Hash, u64> {
        let mut weights: std::collections::HashMap<Hash, u64> = std::collections::HashMap::new();

        // Initialize all vertices with weight 1
        for id in dag.transaction_ids() {
            weights.insert(id, 1);
        }

        // For each vertex, add its weight to all ancestors
        for id in dag.transaction_ids() {
            if let Some(_vertex) = dag.get(&id) {
                let mut stack = vec![id];
                let mut visited = std::collections::HashSet::new();

                while let Some(current) = stack.pop() {
                    if !visited.insert(current) {
                        continue;
                    }
                    if current != id {
                        if let Some(w) = weights.get_mut(&current) {
                            *w += 1;
                        }
                    }
                    if let Some(v) = dag.get(&current) {
                        for parent in v.parents() {
                            if !parent.is_zero() {
                                stack.push(*parent);
                            }
                        }
                    }
                }
            }
        }

        weights
    }

    /// Get the confirmation score of a transaction (0.0 to 1.0)
    /// 1.0 means the transaction is fully confirmed (final)
    pub fn confirmation_score(cumulative_weight: u64) -> f64 {
        let threshold = crate::FINALITY_THRESHOLD as f64;
        (cumulative_weight as f64 / threshold).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;
    use crate::dag::transaction::Transaction;
    use crate::dag::vertex::DagVertex;

    #[test]
    fn test_weight_calculation() {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let genesis_id = genesis.id;
        let mut dag = Dag::new();
        dag.insert(DagVertex::new(genesis, 0)).unwrap();

        let tx1 = Transaction::relay_reward(&kp, 100, [genesis_id, genesis_id], 1);
        dag.insert(DagVertex::new(tx1, 1)).unwrap();

        let weights = WeightCalculator::calculate_all_weights(&dag);
        // Genesis should have weight 2 (1 own + 1 from tx1)
        assert_eq!(*weights.get(&genesis_id).unwrap(), 2);
    }

    #[test]
    fn test_confirmation_score() {
        assert_eq!(WeightCalculator::confirmation_score(0), 0.0);
        assert!(WeightCalculator::confirmation_score(5) > 0.0);
        assert_eq!(
            WeightCalculator::confirmation_score(crate::FINALITY_THRESHOLD),
            1.0
        );
        assert_eq!(
            WeightCalculator::confirmation_score(crate::FINALITY_THRESHOLD + 100),
            1.0
        );
    }
}
