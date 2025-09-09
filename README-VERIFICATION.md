# FLII Marketplace Smart Contract

## Program Verification Information

### Program Details
- **Program ID**: `3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF`
- **Network**: Solana Mainnet
- **Deployment Date**: September 9, 2024
- **Authority**: `Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY`

### Build Instructions

#### Prerequisites
- Rust 1.75.0
- Solana CLI 1.17.0
- Anchor CLI 0.29.0

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/fliidotdev/flii-marketplace-contract.git
cd flii-marketplace-contract

# Install dependencies
npm install

# Build the program
anchor build

# The built program will be at:
# target/deploy/marketplace.so
```

#### Verify Build
```bash
# Compare with deployed program
solana program dump 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF deployed.so
shasum -a 256 target/deploy/marketplace.so
shasum -a 256 deployed.so
# The hashes should match
```

### Contract Features
- Component marketplace for web developers
- All transactions in FLII tokens (BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump)
- 2.5% platform fee
- 2% creator rewards
- IPFS/Arweave metadata storage

### Security
- Non-custodial design
- Program-owned accounts
- Validated inputs
- Fee limits enforced

### License
MIT

### Contact
- X/Twitter: @yesnodotfun
- Website: https://yes-no.fun
