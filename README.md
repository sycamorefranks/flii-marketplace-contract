# Marketplace Contracts

Solana smart contracts for the FLII.dev component marketplace.

## Programs

### Marketplace Program
Core marketplace functionality:
- Component listing and discovery
- Purchase transactions with automatic revenue distribution
- 70/30 creator/platform split
- Component access control

### Revenue Share Program
Automated revenue distribution:
- Configurable revenue splits
- Batch payment processing
- Real-time creator payouts
- Platform fee collection

### Token Program
$FLII token implementation:
- SPL token with metadata
- Staking mechanisms
- Governance features
- Reward distribution

## Architecture
Component Purchase Flow:

Buyer initiates purchase
Smart contract validates payment
Automatic split: 70% to creator, 30% to platform
Purchase record created on-chain
Access granted to buyer


## Development

### Prerequisites
- Rust 1.75+
- Solana CLI 1.17+
- Anchor 0.29+
- Node.js 18+

### Setup
```bash
# Install dependencies
yarn install

# Build programs
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
Local Development
bash# Start local validator
solana-test-validator

# Deploy locally
anchor deploy --provider.cluster localnet

# Run integration tests
yarn test:integration
SDK Usage
typescriptimport { MarketplaceClient } from '@fliidotdev/marketplace-sdk';
import { Connection, Keypair } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com');
const wallet = Keypair.generate();
const client = new MarketplaceClient(connection, wallet, PROGRAM_ID);

// List a component
await client.listComponent(
  'component-001',
  100_000_000n, // 0.1 SOL
  'ipfs://metadata'
);

// Purchase a component
await client.purchaseComponent(
  'component-001',
  componentPubkey,
  creatorTokenAccount,
  marketplaceTokenAccount
);
Contract Addresses
Devnet

Marketplace: FLiixxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Revenue Share: FLiiRSxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Token: FLiiTKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

Mainnet

TBD

Security

All contracts audited by [Audit Firm]
Bug bounty program active
Formal verification completed

License
MIT
