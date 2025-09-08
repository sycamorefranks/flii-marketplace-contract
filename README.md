<div align="center">
  <h1>
    <br/>
    <img src="https://raw.githubusercontent.com/fliidotdev/marketplace-contracts/main/assets/logo.svg" alt="Marketplace" width="200"/>
    <br/>
    NFT Marketplace Smart Contracts
    <br/>
  </h1>

  <h3>Solana Programs for NFT Trading and Component Marketplace</h3>

  <p>
    <a href="https://github.com/fliidotdev/marketplace-contracts/actions"><img src="https://img.shields.io/badge/build-passing-brightgreen?style=flat-square" alt="Build Status" /></a>
    <a href="https://github.com/fliidotdev/marketplace-contracts"><img src="https://img.shields.io/badge/tests-passing-brightgreen?style=flat-square" alt="Tests" /></a>
    <a href="https://github.com/fliidotdev/marketplace-contracts"><img src="https://img.shields.io/badge/audit-pending-yellow?style=flat-square" alt="Audit" /></a>
    <a href="https://github.com/fliidotdev/marketplace-contracts/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License" /></a>
    <a href="https://twitter.com/fliidotdev"><img src="https://img.shields.io/badge/X-@fliidotdev-black?style=flat-square&logo=x" alt="X (Twitter)" /></a>
  </p>
</div>

---

## 🚀 Overview

Professional Solana smart contracts powering NFT and component marketplace functionality with advanced features including escrow management, auction systems, and fee distribution.

### Features

- 🔒 **Secure Escrow** - PDA-based escrow accounts for safe trading
- 🎯 **Auction System** - Time-based auctions with automatic bid refunds
- 💰 **Fee Management** - Platform and creator fee distribution
- 📝 **Offer System** - Make and accept offers on listings
- ⚡ **Optimized** - Gas-efficient program design
- 🛡️ **Battle-tested** - Comprehensive test coverage

## 📦 Programs

### Marketplace Program
- NFT listing creation and management
- Fixed price and auction listings
- Bid placement and automatic refunds
- Offer and counter-offer system
- Fee distribution mechanism

### Component Registry
- Component listing and discovery
- Version management
- License verification
- Usage tracking

## 🛠️ Development

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### Build & Test
```bash
# Build programs
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## 🔐 Security

Security audit in progress. Bug reports: security@fliidotdev.com

## 📄 License

MIT © [FLII.dev](https://flii.dev)

## 🔗 Links

- [Documentation](https://docs.fliidotdev.com/marketplace)
- [Website](https://flii.dev)
- [X (Twitter)](https://twitter.com/fliidotdev)
