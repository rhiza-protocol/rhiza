<div align="center">

<img src="https://raw.githubusercontent.com/rhiza-protocol/rhiza/main/rhiza-logo.png" alt="Rhiza Protocol" width="180">

# Rhiza Protocol

### No Blockchain. No Mining. No Staking. Just Currency.

[![Build](https://img.shields.io/badge/build-passing-10b981?style=flat-square)](https://github.com/rhiza-protocol/rhiza)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-ef4444?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-10b981?style=flat-square)](CONTRIBUTING.md)
[![Website](https://img.shields.io/badge/web-rhiza--protocol.github.io-8b5cf6?style=flat-square)](https://rhiza-protocol.github.io)

**Rhiza is a DAG-based, feeless, mesh-networked cryptocurrency that works without internet.**

[Website](https://rhiza-protocol.github.io) · [Whitepaper](WHITEPAPER.md) · [Quick Start](#-quick-start) · [Contributing](CONTRIBUTING.md)

</div>

---

## The Problem

Crypto was supposed to be decentralized. It isn't.

- **Bitcoin**: 3 mining pools control 65%+ of hashrate
- **Ethereum**: PoS lets whales dominate governance
- **Every chain**: Requires internet. Charges fees. Processes transactions one block at a time.

**Rhiza fixes all of this.**

## How Rhiza Works

<table>
<tr>
<td width="50%" valign="top">

### 🔗 DAG, Not Blockchain

Rhiza uses a **Directed Acyclic Graph** instead of a blockchain. Every transaction references 2 previous transactions, creating a web of validations:

```
    TX-1    TX-2
     \\    //
      TX-3  TX-4
       \\  //\\
       TX-5  TX-6
        \\  //
         TX-7
```

**Result**: Parallel processing, instant finality, and a network that gets faster with more users.

</td>
<td width="50%" valign="top">

### ⚡ Proof of Relay

No mining hardware. No staked capital. You earn RHZ by **relaying transactions**:

1. Receive a transaction from a peer
2. Validate it (signatures, parents, structure)
3. Relay it to your peers
4. Earn a reward

Everyone with a device can participate equally. Rewards have diminishing returns — **no single node can dominate**.

</td>
</tr>
</table>

### 📡 Mesh Networking — Works Without Internet

| Transport | Range | Use Case |
|-----------|-------|----------|
| **TCP/IP** | Global | Standard internet |
| **WiFi Direct** | ~100m | Local peer-to-peer |
| **Bluetooth LE** | ~30m | Phone-to-phone |
| **LoRa** | ~15km | Rural/disaster areas |

Transactions propagate through whatever network is available. **Internet goes down? Rhiza keeps working.**

## Quick Start

```bash
# Clone & build
git clone https://github.com/rhiza-protocol/rhiza.git
cd rhiza && cargo build --release

# Create your wallet
cargo run --bin rhiza-cli -- wallet create
# 🌿 Wallet Created!
# 📍 Address: rhz1qw508d6qejxtdg4y5r3z...

# Start your node + wallet UI
cargo run --bin rhiza-node -- start
# 🌐 Wallet UI → http://localhost:7471
```

**That's it.** No syncing gigabytes of chain data. No buying tokens to pay gas fees. No setting up mining hardware.

## Architecture

```
rhiza/
├── rhiza-core/          # Core protocol library
│   ├── crypto/          # Ed25519 signatures, BLAKE3 hashing
│   ├── dag/             # DAG structure, transactions, validation
│   ├── consensus/       # Proof of Relay, cumulative weight finality
│   ├── network/         # Gossip protocol, mesh networking
│   └── wallet/          # Bech32m addresses, keystore
├── rhiza-node/          # Full node daemon with REST API + Wallet UI
├── rhiza-cli/           # Command-line wallet & tools
└── WHITEPAPER.md        # Full technical specification
```

### Tech Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| **Language** | Rust | Memory safety, zero-cost abstractions, no GC |
| **Hashing** | BLAKE3 | Fastest secure hash (3x faster than SHA-256) |
| **Signatures** | Ed25519 | Battle-tested, 64-byte compact signatures |
| **Addresses** | Bech32m | Human-readable, typo-detecting (`rhz1...`) |
| **P2P** | libp2p | Production-grade peer-to-peer networking |
| **Serialization** | bincode | Compact, deterministic binary encoding |

## Protocol Specification

| Parameter | Value |
|-----------|-------|
| **Ticker** | `RHZ` |
| **Max Supply** | 21,000,000 RHZ |
| **Smallest Unit** | 10⁻⁸ RHZ (1 satoshi equivalent) |
| **Transaction Fees** | **0** — always free |
| **Base Relay Reward** | 0.01 RHZ per relay |
| **Halving Interval** | Every 1,000 relays |
| **Finality** | Cumulative weight ≥ 10 |
| **Parent References** | 2 per transaction |
| **Consensus** | Proof of Relay (PoR) |
| **Default Port** | 7470 |
| **Address Prefix** | `rhz1` |

## Wallet UI

The node includes a built-in web wallet at `http://localhost:7471`:

- 💰 Real-time balance display
- 📤 Send RHZ to any address
- 📥 Receive with one-click address copy
- 📊 Live DAG statistics
- 📋 Full transaction history
- 🔄 Relay reward claiming

## Why Not Just Use Bitcoin/Ethereum/Solana?

| | Bitcoin | Ethereum | Solana | **Rhiza** |
|---|---|---|---|---|
| **Consensus** | PoW (wasteful) | PoS (plutocratic) | PoH+PoS | **PoR (egalitarian)** |
| **Fees** | $1-50 | $0.50-100 | $0.001 | **$0 always** |
| **Finality** | ~60 min | ~15 min | ~0.4s | **Seconds** |
| **Min. Hardware** | ASIC ($5000+) | 32 ETH ($100k+) | High-spec server | **Any device** |
| **Works Offline** | ❌ | ❌ | ❌ | **✅ Mesh** |
| **Private** | ❌ Pseudonymous | ❌ Pseudonymous | ❌ Pseudonymous | **✅ ZK-ready** |

## Security

- **BLAKE3** — Cryptographic hashing, 3x faster than SHA-256, equivalent security
- **Ed25519** — Elliptic curve signatures (same as Signal, Tor, SSH)
- **Cumulative weight** — Finality without a single block producer
- **Diminishing returns** — Sybil resistance built into the reward curve
- **Bech32m** — Addresses with built-in error detection

## Roadmap

- [x] Core protocol (DAG, PoR, signatures, mesh)
- [x] CLI wallet & node daemon
- [x] Web wallet UI
- [x] Whitepaper
- [ ] Persistent storage (sled DB)
- [ ] Multi-node mesh networking
- [ ] Zero-knowledge transaction privacy
- [ ] Mobile wallet (iOS/Android)
- [ ] Browser extension wallet
- [ ] LoRa transport layer
- [ ] Smart contract layer (Rhiza VM)

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

```bash
# Run tests
cargo test --workspace

# Build in release mode
cargo build --release
```

All 47 unit tests pass across crypto, DAG, consensus, networking, and wallet modules.

## Community

- 🌐 [Website](https://rhiza-protocol.github.io)
- 📄 [Whitepaper](WHITEPAPER.md)
- 🐛 [Issues](https://github.com/rhiza-protocol/rhiza/issues)

## License

MIT License — Free as in freedom. See [LICENSE](LICENSE) for details.

---

<div align="center">

**Rhiza** (ρίζα) — Greek for "root"

*The root system grows beneath the surface. By the time you see it, it's everywhere.*

⭐ **Star this repo** if you believe crypto should be truly decentralized.

</div>
