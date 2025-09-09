```text
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║  ███╗   ███╗ █████╗ ██████╗ ██╗  ██╗███████╗████████╗██████╗ ██╗      █████╗  ██████╗███████╗
║  ████╗ ████║██╔══██╗██╔══██╗██║ ██╔╝██╔════╝╚══██╔══╝██╔══██╗██║     ██╔══██╗██╔════╝██╔════╝
║  ██╔████╔██║███████║██████╔╝█████╔╝ █████╗     ██║   ██████╔╝██║     ███████║██║     █████╗  
║  ██║╚██╔╝██║██╔══██║██╔══██╗██╔═██╗ ██╔══╝     ██║   ██╔═══╝ ██║     ██╔══██║██║     ██╔══╝  
║  ██║ ╚═╝ ██║██║  ██║██║  ██║██║  ██╗███████╗   ██║   ██║     ███████╗██║  ██║╚██████╗███████╗
║  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝╚══════╝
║                                                                              ║
║                      Solana NFT Marketplace Smart Contracts                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=flat-square)](https://github.com/fliidotdev/marketplace-contracts/actions)
[![npm version](https://img.shields.io/npm/v/@fliidotdev/marketplace-sdk.svg?style=flat-square)](https://www.npmjs.com/package/@fliidotdev/marketplace-sdk)
[![Anchor Version](https://img.shields.io/badge/anchor-0.29.0-blue?style=flat-square)](https://www.anchor-lang.com/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/fliidotdev/marketplace-contracts/blob/main/LICENSE)

---

## Overview

**Marketplace Contracts** is a comprehensive NFT marketplace infrastructure built on Solana. It provides all the smart contracts and tools needed to launch a fully-featured NFT marketplace with advanced trading features, royalty enforcement, and seamless integration with Solana's NFT standards.

### Key Features

- **NFT Trading**: Buy, sell, and auction NFTs with multiple payment options
- **Collection Management**: Create and manage NFT collections with metadata
- **Royalty Enforcement**: Built-in creator royalties with on-chain enforcement
- **Auction System**: English and Dutch auction mechanisms
- **Offer System**: Make and accept offers on NFTs
- **Bundle Sales**: Sell multiple NFTs as a bundle
- **Escrow**: Secure escrow system for trustless transactions
- **Fee Management**: Configurable marketplace and creator fees

## Installation

### Prerequisites

- Rust 1.70+
- Solana CLI 1.17+
- Anchor CLI 0.29.0+
- Node.js 18+

### Smart Contracts

```bash
# Clone repository
git clone https://github.com/fliidotdev/marketplace-contracts.git
cd marketplace-contracts

# Install dependencies
npm install

# Build programs
anchor build

# Run tests
anchor test
```

### TypeScript SDK

```bash
npm install @fliidotdev/marketplace-sdk
```

## Architecture

### Core Programs

#### Marketplace Program

Main marketplace logic for listings, sales, and offers.

```rust
// Program ID: MktpXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u16,
    pub total_volume: u64,
    pub total_sales: u64,
    pub paused: bool,
}

pub struct Listing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub payment_mint: Option<Pubkey>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}
```

**Key Instructions:**
- `create_marketplace`: Initialize a new marketplace
- `list_nft`: Create a new NFT listing
- `buy_nft`: Purchase a listed NFT
- `cancel_listing`: Cancel an active listing
- `make_offer`: Make an offer on an NFT
- `accept_offer`: Accept an offer

#### Auction Program

Handles auction mechanics for NFTs.

```rust
// Program ID: AuctXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

pub struct Auction {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub start_price: u64,
    pub reserve_price: Option<u64>,
    pub min_bid_increment: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub highest_bid: Option<Bid>,
    pub auction_type: AuctionType,
}

pub enum AuctionType {
    English,
    Dutch { price_drop_per_second: u64 },
}
```

**Key Instructions:**
- `create_auction`: Start a new auction
- `place_bid`: Place a bid on an auction
- `settle_auction`: Finalize auction and transfer NFT
- `cancel_auction`: Cancel an auction (if no bids)

#### Collection Program

Manages NFT collections and metadata.

```rust
// Program ID: CollXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

pub struct Collection {
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub royalty_basis_points: u16,
    pub verified: bool,
    pub total_supply: u64,
}
```

## Usage Examples

### Creating a Listing

```typescript
import { MarketplaceSDK } from '@fliidotdev/marketplace-sdk';

const sdk = new MarketplaceSDK(connection, wallet);

// List an NFT
const listing = await sdk.createListing({
  nftMint: new PublicKey('...'),
  price: 5 * LAMPORTS_PER_SOL,
  paymentMint: null, // SOL payment
  duration: 7 * 24 * 60 * 60 // 7 days
});

console.log('Listing created:', listing.address);
```

### Buying an NFT

```typescript
// Buy a listed NFT
const purchase = await sdk.buyNFT({
  listing: listingAddress,
  buyer: wallet.publicKey
});

console.log('Purchase signature:', purchase.signature);
```

### Creating an Auction

```typescript
// Create English auction
const auction = await sdk.createAuction({
  nftMint: nftMint,
  startPrice: 1 * LAMPORTS_PER_SOL,
  reservePrice: 5 * LAMPORTS_PER_SOL,
  minBidIncrement: 0.1 * LAMPORTS_PER_SOL,
  duration: 24 * 60 * 60, // 24 hours
  auctionType: 'english'
});

// Place a bid
await sdk.placeBid({
  auction: auction.address,
  amount: 1.5 * LAMPORTS_PER_SOL
});
```

### Making Offers

```typescript
// Make an offer on any NFT
const offer = await sdk.makeOffer({
  nftMint: nftMint,
  offerAmount: 3 * LAMPORTS_PER_SOL,
  expiration: Date.now() + 24 * 60 * 60 * 1000
});

// Seller accepts offer
await sdk.acceptOffer({
  offer: offer.address
});
```

## Advanced Features

### Bundle Sales

```typescript
// Create a bundle listing
const bundle = await sdk.createBundle({
  nfts: [nft1, nft2, nft3],
  price: 10 * LAMPORTS_PER_SOL,
  description: 'Rare collection bundle'
});
```

### Royalty Distribution

```typescript
// Automatic royalty distribution on sale
const sale = await sdk.buyNFT({
  listing: listingAddress,
  // Royalties automatically sent to creators
});
```

### Collection Verification

```typescript
// Verify a collection (admin only)
await sdk.verifyCollection({
  collection: collectionAddress,
  verified: true
});
```

## Testing

```bash
# Run all tests
npm test

# Run specific test suite
npm test -- marketplace

# Run with coverage
npm run test:coverage

# Test on devnet
anchor test --provider.cluster devnet
```

## Deployment

### Local Deployment

```bash
# Start local validator
solana-test-validator

# Deploy programs
anchor deploy

# Initialize marketplace
npm run init:marketplace
```

### Mainnet Deployment

```bash
# Build verifiable
anchor build --verifiable

# Deploy to mainnet
anchor deploy --provider.cluster mainnet

# Verify build
anchor verify <PROGRAM_ID>
```

## SDK Documentation

### MarketplaceSDK

```typescript
class MarketplaceSDK {
  constructor(connection: Connection, wallet: Wallet);
  
  // Listing methods
  createListing(params: CreateListingParams): Promise<Listing>;
  cancelListing(listing: PublicKey): Promise<TransactionSignature>;
  buyNFT(params: BuyNFTParams): Promise<Purchase>;
  
  // Auction methods
  createAuction(params: CreateAuctionParams): Promise<Auction>;
  placeBid(params: PlaceBidParams): Promise<Bid>;
  settleAuction(auction: PublicKey): Promise<TransactionSignature>;
  
  // Offer methods
  makeOffer(params: MakeOfferParams): Promise<Offer>;
  acceptOffer(offer: PublicKey): Promise<TransactionSignature>;
  cancelOffer(offer: PublicKey): Promise<TransactionSignature>;
  
  // Query methods
  getListings(filter?: ListingFilter): Promise<Listing[]>;
  getAuctions(filter?: AuctionFilter): Promise<Auction[]>;
  getOffers(nft: PublicKey): Promise<Offer[]>;
}
```

## Fee Structure

```typescript
// Default fee structure
const fees = {
  marketplaceFee: 2.5, // 2.5% to marketplace
  creatorRoyalty: 5.0, // 5% to original creator
  minListingPrice: 0.01 * LAMPORTS_PER_SOL
};
```

## Security

### Features

- Non-custodial architecture
- Escrow-based transactions
- Signature verification
- Access control lists
- Rate limiting
- Emergency pause functionality

### Audits

- Internal security review: ✅
- External audit: Scheduled

## Program Addresses

### Mainnet

```
Marketplace: MktpXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Auction: AuctXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Collection: CollXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### Devnet

```
Marketplace: DevMktpXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Auction: DevAuctXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Collection: DevCollXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

## Contributing

We welcome contributions! Please reach out on [X/Twitter](https://twitter.com/fliidotdev) for details.

### Development

```bash
# Setup dev environment
./scripts/setup.sh

# Run local marketplace
./scripts/start-local.sh

# Run integration tests
./scripts/test-integration.sh
```

## License

MIT License - see [LICENSE](./LICENSE) for details.

## Links

- [Documentation](https://flii.dev/docs)
- [API Reference](https://flii.dev/docs)
- [GitHub](https://github.com/fliidotdev)
- [X/Twitter](https://twitter.com/fliidotdev)

---

Built by the FLII.dev team
