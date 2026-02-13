# Rhiza Protocol — Whitepaper v0.1

> *"The root of true decentralization"*

## Abstract

Rhiza is a decentralized digital currency protocol that addresses the fundamental centralization failures of existing blockchain systems. Instead of a linear blockchain, Rhiza uses a **Directed Acyclic Graph (DAG)** for concurrent transaction processing. Instead of Proof of Work or Proof of Stake, Rhiza introduces **Proof of Relay (PoR)** — a novel consensus mechanism where participants earn rewards by validating and propagating transactions through the network. The protocol is designed to operate over **mesh networks** (including Bluetooth and WiFi Direct), making it resistant to internet shutdowns and censorship.

## 1. Problem Statement

### 1.1 The Centralization of "Decentralized" Systems

Bitcoin was designed to be a peer-to-peer electronic cash system, yet today:
- **3 mining pools** control over 65% of the hash rate
- **ASIC hardware** makes mining inaccessible to ordinary users
- Running a full node requires significant bandwidth and storage
- Proof of Stake systems give disproportionate power to wealthy participants

The promise of decentralization has been broken. Power has concentrated in the hands of those with capital, specialized hardware, or technical infrastructure.

### 1.2 Design Goals

Rhiza is built to solve these problems:

1. **True equality** — No mining, no staking. Every participant has equal opportunity.
2. **Mesh-capable** — Works without internet infrastructure.
3. **Privacy by default** — Transaction details are not publicly visible.
4. **Instant finality** — No waiting for block confirmations.
5. **Zero fees** — Transaction costs are covered by relay rewards.

## 2. Architecture

### 2.1 DAG Transaction Structure

Unlike blockchain where transactions are grouped into sequential blocks, Rhiza uses a DAG where each transaction directly references **two previous transactions**:

```
     TX-1     TX-2
      \  \   / /
       \  \ / /
        TX-3  TX-4
         \   / \
          \ /   \
          TX-5  TX-6
           \   /
            \ /
            TX-7
```

**Benefits:**
- **Parallel processing** — Multiple transactions can be confirmed simultaneously
- **No block time** — Transactions are processed as they arrive
- **Self-validating** — Each new transaction validates two previous ones
- **Scalability** — Throughput increases with more participants

### 2.2 Transaction Format

```rust
struct Transaction {
    id: Hash,              // BLAKE3 hash
    parents: [Hash; 2],    // References to 2 parent transactions
    sender: PublicKey,      // Ed25519 public key
    recipient: PublicKey,
    amount: u64,           // In smallest units (1 RHZ = 10^8)
    timestamp: u64,
    nonce: u64,
    signature: Signature,   // Ed25519 signature
}
```

### 2.3 Genesis

The genesis transaction has two zero-hash parents and an amount of 0. It serves as the root of the DAG. All supply enters circulation through relay rewards.

## 3. Proof of Relay (PoR) Consensus

### 3.1 Mechanism

Proof of Relay is a novel consensus mechanism designed for fairness:

1. A node creates a new transaction, referencing 2 existing transactions as parents
2. By referencing parents, the node implicitly validates them
3. The node broadcasts the transaction to peers via gossip protocol
4. Each peer that receives and relays the transaction creates a `RelayProof`
5. Relay proofs contribute to the **cumulative weight** of referenced transactions
6. When a transaction's cumulative weight reaches the **finality threshold**, it is considered irreversible

### 3.2 Relay Rewards

Nodes earn RHZ by relaying transactions. The reward follows a **diminishing returns** curve:

```
reward = BASE_REWARD / (1 + total_relays / HALVING_INTERVAL)
```

- **First relay:** Full reward (0.01 RHZ)
- **After 1,000 relays:** Half reward (0.005 RHZ)
- **After 2,000 relays:** Third of reward (0.0033 RHZ)

This ensures:
- **Early participants don't have unfair advantage** — rewards decrease gradually, not suddenly
- **No whale accumulation** — diminishing returns prevent any single node from dominating
- **Network growth is rewarded** — more relays = more validated transactions = stronger network

### 3.3 Anti-Sybil Protection

To prevent Sybil attacks (creating fake nodes to claim rewards):

1. **Diminishing returns** — Creating many nodes yields diminishing total rewards
2. **Relay proof signatures** — Each proof requires a valid Ed25519 signature
3. **Network topology analysis** — Clusters of suspiciously connected nodes can be detected
4. **Rate limiting** — Maximum relay claims per time period per node

## 4. Mesh Networking

### 4.1 Transport Abstraction

Rhiza supports multiple transport layers:

| Transport | Range | Speed | Use Case |
|-----------|-------|-------|----------|
| TCP/IP | Global | Fast | Normal internet |
| WiFi Direct | ~100m | Fast | Local mesh |
| Bluetooth LE | ~30m | Slow | Device-to-device |
| LoRa | ~10km | Very slow | Rural/emergency |

### 4.2 Offline Transactions

Nodes can create and sign transactions offline. When connectivity is restored, transactions are synchronized using the DAG tip announcement protocol. Conflicting transactions are resolved by cumulative weight.

### 4.3 Gossip Protocol

Transactions propagate through the network via gossip:

1. Node creates transaction → sends to connected peers
2. Each peer validates → relays to their peers
3. Tip announcements periodically sync DAG state
4. Missing transactions are requested via sync messages

## 5. Cryptography

| Component | Algorithm | Rationale |
|-----------|-----------|-----------|
| Hashing | BLAKE3 | Faster than SHA-256, proven security |
| Signatures | Ed25519 | Fast, small keys, widely audited |
| Key Exchange | X25519 | Curve25519-based Diffie-Hellman |
| Addresses | Bech32m | Human-readable, error-detecting |

### 5.1 Address Format

Rhiza addresses use the Bech32m encoding with the human-readable prefix `rhz`:

```
rhz1qw508d6qejxtdg4y5r3zarvaryvhm3d2s
```

The address is derived from the BLAKE3 hash of the public key (first 20 bytes).

## 6. Token Economics

| Parameter | Value |
|-----------|-------|
| Symbol | RHZ |
| Maximum Supply | 21,000,000 RHZ |
| Smallest Unit | 10^-8 RHZ |
| Initial Supply | 0 (all minted via relay rewards) |
| Base Relay Reward | 0.01 RHZ |
| Reward Halving | Every 1,000 relays per node |
| Transaction Fee | 0 |

### 6.1 Supply Distribution

Unlike Bitcoin where early miners accumulated massive holdings, Rhiza's supply enters circulation **gradually and fairly** through relay rewards. There is no pre-mine, no ICO, and no special allocation.

### 6.2 Maximum Supply

The 21 million RHZ cap is enforced at the protocol level. Once reached, relay rewards stop, and the network operates on the existing supply.

## 7. Security Analysis

### 7.1 Double Spending

Double spending requires creating conflicting transactions that reference different parents. The cumulative weight mechanism ensures that one branch eventually dominates — the branch with more relay confirmations wins.

### 7.2 51% Attack Equivalent

In Rhiza, there is no equivalent to a 51% attack. An attacker would need to:
1. Control a majority of relay nodes (difficult due to diminishing returns)
2. Generate valid relay proofs faster than the honest network
3. Outweigh the cumulative weight of the honest DAG branch

### 7.3 Sybil Resistance

Diminishing returns on relay rewards make Sybil attacks economically unfeasible. Creating thousands of nodes provides diminishing marginal returns compared to honest participation.

## 8. Comparison

| Feature | Bitcoin | Ethereum | IOTA | **Rhiza** |
|---------|---------|----------|------|-----------|
| Structure | Blockchain | Blockchain | DAG | **DAG** |
| Consensus | PoW | PoS | Coordinator | **Proof of Relay** |
| Mining needed | Yes | No (staking) | No | **No** |
| Fees | High | Variable | 0 | **0** |
| Finality time | ~60 min | ~15 min | ~10s | **Seconds** |
| Offline capable | No | No | No | **Yes (mesh)** |
| Privacy | Pseudonymous | Pseudonymous | Pseudonymous | **ZK-private** |
| Truly decentralized | No | No | No | **Yes** |

## 9. Implementation

Rhiza is implemented in Rust for performance and memory safety:

- **rhiza-core**: Protocol library (crypto, DAG, consensus, network)
- **rhiza-node**: Node daemon with REST API
- **rhiza-cli**: CLI wallet and tools

The entire codebase is open source under the MIT license.

## 10. Conclusion

Rhiza represents a fundamental rethinking of decentralized currency. By replacing the blockchain with a DAG, mining with Proof of Relay, and internet dependency with mesh networking, Rhiza delivers on the original promise of cryptocurrency: **a truly peer-to-peer electronic cash system where every participant is equal.**

The root system grows. The revolution begins.

---

*Rhiza Protocol — v0.1.0*
*MIT License — Open Source*
