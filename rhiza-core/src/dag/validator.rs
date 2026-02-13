use crate::dag::transaction::{Transaction, TransactionType};
use crate::dag::vertex::Dag;

/// Validates transactions before they are added to the DAG
pub struct TransactionValidator;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid transaction ID")]
    InvalidId,
    #[error("insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u64, need: u64 },
    #[error("zero amount transfer")]
    ZeroAmount,
    #[error("amount exceeds maximum supply")]
    ExceedsMaxSupply,
    #[error("parent transaction not found")]
    ParentNotFound,
    #[error("self-referencing parents")]
    SelfReference,
    #[error("relay reward exceeds allowed amount")]
    InvalidRelayReward,
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
}

impl TransactionValidator {
    /// Validate a transaction against the current DAG state
    pub fn validate(tx: &Transaction, dag: &Dag) -> Result<(), ValidationError> {
        // 1. Verify transaction ID
        if !tx.verify_id() {
            return Err(ValidationError::InvalidId);
        }

        // 2. Verify signature
        if !tx.verify_signature() {
            return Err(ValidationError::InvalidSignature);
        }

        // 3. Type-specific validation
        match tx.data.tx_type {
            TransactionType::Genesis => Self::validate_genesis(tx, dag),
            TransactionType::Transfer => Self::validate_transfer(tx, dag),
            TransactionType::RelayReward => Self::validate_relay_reward(tx, dag),
        }
    }

    fn validate_genesis(tx: &Transaction, dag: &Dag) -> Result<(), ValidationError> {
        // Genesis is only valid if there's no existing genesis
        if dag.genesis_id.is_some() {
            return Err(ValidationError::InvalidId);
        }
        // Genesis must reference zero hashes
        if !tx.data.parents[0].is_zero() || !tx.data.parents[1].is_zero() {
            return Err(ValidationError::ParentNotFound);
        }
        Ok(())
    }

    fn validate_transfer(tx: &Transaction, dag: &Dag) -> Result<(), ValidationError> {
        // Amount must be > 0
        if tx.data.amount == 0 {
            return Err(ValidationError::ZeroAmount);
        }

        // Amount must not exceed max supply
        if tx.data.amount > crate::MAX_SUPPLY {
            return Err(ValidationError::ExceedsMaxSupply);
        }

        // Parents must exist in DAG
        for parent in &tx.data.parents {
            if dag.get(parent).is_none() {
                return Err(ValidationError::ParentNotFound);
            }
        }

        // Check balance
        let balance = dag.get_balance(&tx.data.sender);
        let total_needed = tx.data.amount + tx.data.fee;
        if balance < total_needed {
            return Err(ValidationError::InsufficientBalance {
                have: balance,
                need: total_needed,
            });
        }

        Ok(())
    }

    fn validate_relay_reward(tx: &Transaction, dag: &Dag) -> Result<(), ValidationError> {
        // Recipient must be the sender (self-reward)
        if tx.data.sender != tx.data.recipient {
            return Err(ValidationError::InvalidRelayReward);
        }

        // Parents must exist
        for parent in &tx.data.parents {
            if dag.get(parent).is_none() {
                return Err(ValidationError::ParentNotFound);
            }
        }

        // Reward amount must be within allowed range
        let max_reward = crate::BASE_RELAY_REWARD;
        if tx.data.amount > max_reward {
            return Err(ValidationError::InvalidRelayReward);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyPair;
    use crate::dag::vertex::DagVertex;

    fn create_dag_with_balance() -> (Dag, KeyPair) {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let genesis_id = genesis.id;
        let mut dag = Dag::new();
        dag.insert(DagVertex::new(genesis, 0)).unwrap();

        // Add relay reward to give the keypair some balance
        let reward = Transaction::relay_reward(&kp, 1_000_000, [genesis_id, genesis_id], 1);
        dag.insert(DagVertex::new(reward, 1)).unwrap();

        (dag, kp)
    }

    #[test]
    fn test_validate_genesis() {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let dag = Dag::new();
        assert!(TransactionValidator::validate(&genesis, &dag).is_ok());
    }

    #[test]
    fn test_validate_duplicate_genesis() {
        let kp = KeyPair::generate();
        let genesis = Transaction::genesis(&kp);
        let mut dag = Dag::new();
        dag.insert(DagVertex::new(genesis, 0)).unwrap();

        let genesis2 = Transaction::genesis(&KeyPair::generate());
        assert!(TransactionValidator::validate(&genesis2, &dag).is_err());
    }

    #[test]
    fn test_validate_transfer() {
        let (dag, sender) = create_dag_with_balance();
        let recipient = KeyPair::generate();
        let parents = dag.select_parents();

        let tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            500_000,
            parents,
            2,
        );

        assert!(TransactionValidator::validate(&tx, &dag).is_ok());
    }

    #[test]
    fn test_validate_insufficient_balance() {
        let (dag, sender) = create_dag_with_balance();
        let recipient = KeyPair::generate();
        let parents = dag.select_parents();

        let tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            999_999_999, // Way more than balance
            parents,
            2,
        );

        assert!(matches!(
            TransactionValidator::validate(&tx, &dag),
            Err(ValidationError::InsufficientBalance { .. })
        ));
    }

    #[test]
    fn test_validate_relay_reward() {
        let (dag, kp) = create_dag_with_balance();
        let parents = dag.select_parents();

        let tx = Transaction::relay_reward(&kp, 500_000, parents, 3);
        assert!(TransactionValidator::validate(&tx, &dag).is_ok());
    }

    #[test]
    fn test_validate_tampered_transaction() {
        let (dag, sender) = create_dag_with_balance();
        let recipient = KeyPair::generate();
        let parents = dag.select_parents();

        let mut tx = Transaction::transfer(
            &sender,
            recipient.public_key,
            100,
            parents,
            2,
        );
        tx.data.amount = 999_999; // Tamper
        assert!(TransactionValidator::validate(&tx, &dag).is_err());
    }
}
