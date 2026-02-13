<div align="center">

# 🌿 Rhiza Protocol

### The Root of True Decentralization

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

*A revolutionary decentralized currency that actually delivers on the promise of decentralization.*

</div>

---

## ❓ Why Rhiza?

Bitcoin promised decentralization but delivered mining pools. Ethereum promised it with staking but created plutocracy. **Rhiza (ρίζα — Greek for "root") goes back to the root of what decentralization should be.**

| Problem | Old Crypto | Rhiza |
|---------|-----------|-------|
| Mining centralization | 3 pools control 65%+ | **No mining at all** |
| Rich get richer | PoS favors whales | **Equal participation** |
| Slow & expensive | Block times, high fees | **Instant, zero fees** |
| Internet dependent | Requires connectivity | **Mesh networking** |
| No privacy | Public ledger | **ZK-private by default** |

## 🏗️ Architecture

### DAG Instead of Blockchain

```
    TX-1    TX-2
     \\    //
      TX-3  TX-4
       \\  //\\
       TX-5  TX-6
        \\  //
         TX-7
```

Every transaction references **2 previous transactions**, creating a web of validations instead of a single chain. This enables:
- ⚡ **Parallel processing** — thousands of TX per second
- 🚫 **No block waiting** — instant confirmation
- 📈 **Self-scaling** — more users = faster network

### Proof of Relay (PoR)

Instead of mining or staking, you earn RHZ by **relaying transactions**:
1. Receive a transaction from a peer
2. Validate it (check signatures, check parents exist)
3. Relay it to your peers
4. Earn a relay reward

Rewards have **diminishing returns** — no single node can dominate.

### Mesh Networking

Rhiza works over:
- 🌐 TCP/IP (internet)
- 📶 WiFi Direct
- 📱 Bluetooth LE
- 📡 LoRa (long range radio)

**Even without internet, Rhiza works.**

## 🚀 Quick Start

### Prerequisites

- [Rust 1.70+](https://rustup.rs/)

### Build

```bash
git clone https://github.com/rhiza-protocol/rhiza.git
cd rhiza
cargo build --release
```

### Create a Wallet

```bash
cargo run --bin rhiza-cli -- wallet create
```

Output:
```
  🌿 Rhiza Wallet Created!
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  📍 Address:    rhz1qw508d6qejxtdg4y5r3zarvaryvhm3d2s
  🔑 Public Key: a1b2c3d4...
  📁 Saved to:   ~/.rhiza/wallet.json
```

### Start a Node

```bash
cargo run --bin rhiza-node -- init
cargo run --bin rhiza-node -- start --port 7470
```

### View Protocol Info

```bash
cargo run --bin rhiza-cli -- protocol
```

## 📐 Protocol Constants

| Parameter | Value |
|-----------|-------|
| **Symbol** | RHZ |
| **Max Supply** | 21,000,000 RHZ |
| **Smallest Unit** | 10⁻⁸ RHZ |
| **Base Relay Reward** | 0.01 RHZ |
| **Consensus** | Proof of Relay |
| **Hash Function** | BLAKE3 |
| **Signatures** | Ed25519 |
| **Address Format** | Bech32m (`rhz1...`) |
| **Default Port** | 7470 |

## 📁 Project Structure

```
rhiza/
├── WHITEPAPER.md          # Technical whitepaper
├── rhiza-core/            # Core protocol library
│   └── src/
│       ├── crypto/        # Ed25519, BLAKE3
│       ├── dag/           # DAG transactions & validation
│       ├── consensus/     # Proof of Relay
│       ├── network/       # Gossip & mesh networking
│       └── wallet/        # Keys & addresses
├── rhiza-node/            # Node daemon + REST API
└── rhiza-cli/             # CLI wallet & tools
```

## 🔐 Security

- **BLAKE3** hashing (faster & secure as SHA-256)
- **Ed25519** digital signatures
- **Bech32m** addresses with error detection
- **Cumulative weight** finality (no double spending)
- **Diminishing returns** (anti-Sybil)

## 📜 License

MIT License — Free as in freedom.

## 🤝 Contributing

Rhiza is open source and welcomes contributions. See the [whitepaper](WHITEPAPER.md) for technical details.

---

<div align="center">

*🌿 The root system grows. The revolution begins.*

</div>
