# 🚀 Mainnet Deployment Guide for FLII Marketplace

## Prerequisites

### 1. Required Wallet Setup
- **Authority Wallet**: `Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY`
- **Required SOL Balance**: ~2-3 SOL for deployment
- **FLII Token Mint**: `BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump`

### 2. Keypair File Required
You need the private key/keypair file for the authority wallet. Save it as:
```
~/.config/solana/authority-keypair.json
```

⚠️ **SECURITY WARNING**: Never share or commit your keypair file!

## Deployment Details

### Program IDs
- **Mainnet Program ID**: `3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF`
- **Devnet Program ID**: `6oN2gtocsfZ1ex6WywYrsV6o15iQoqDcqoeK96fhJRiF` (already deployed)

### Contract Features
- ✅ All transactions in FLII tokens
- ✅ 2% staking rewards for creators
- ✅ Configurable platform fees (max 10%)
- ✅ Component listing and purchasing
- ✅ Automatic reward distribution

## Step-by-Step Deployment

### Step 1: Setup Authority Keypair
```bash
# Place your authority wallet keypair at:
# ~/.config/solana/authority-keypair.json

# Verify it's the correct wallet:
solana address --keypair ~/.config/solana/authority-keypair.json
# Should output: Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY
```

### Step 2: Check SOL Balance
```bash
# Check your authority wallet balance
solana balance Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY --url mainnet-beta

# If needed, transfer SOL to the authority wallet
# You need approximately 2-3 SOL for deployment
```

### Step 3: Run Deployment Script
```bash
# Navigate to the marketplace contracts directory
cd /Users/dizrealcastillo/flii-dev/marketplace-contracts

# Run the deployment script
./deploy-mainnet.sh
```

### Step 4: Manual Deployment (Alternative)
If you prefer to deploy manually:

```bash
# 1. Switch to mainnet
solana config set --url mainnet-beta

# 2. Update the program ID in lib.rs
# Replace the declare_id! with: 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

# 3. Build the program
anchor build

# 4. Deploy using authority wallet
solana program deploy \
    target/deploy/marketplace.so \
    --program-id target/deploy/marketplace-mainnet-keypair.json \
    --keypair ~/.config/solana/authority-keypair.json \
    --url mainnet-beta
```

## Post-Deployment Steps

### 1. Initialize the Marketplace
After deployment, you need to initialize the marketplace with:

```typescript
// Using Anchor SDK
const tx = await program.methods.initialize(
    250  // 2.5% platform fee (in basis points)
)
.accounts({
    marketplace: marketplacePDA,
    fliiTokenMint: new PublicKey("BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump"),
    treasuryWallet: treasuryWalletPubkey,
    authority: authorityWallet.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
})
.signers([authorityWallet])
.rpc();
```

### 2. Verify on Explorers
- Solana Explorer: https://explorer.solana.com/address/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
- Solscan: https://solscan.io/account/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
- SolanaFM: https://solana.fm/address/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

### 3. Update Frontend Configuration
Update your frontend to use the mainnet program ID:

```javascript
// config.js
export const MARKETPLACE_PROGRAM_ID = "3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF";
export const FLII_TOKEN_MINT = "BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump";
export const RPC_ENDPOINT = "https://api.mainnet-beta.solana.com";
```

## Cost Breakdown

### Deployment Costs
- **Program Deployment**: ~1.5-2 SOL
- **Account Rent**: ~0.5 SOL
- **Transaction Fees**: ~0.01 SOL
- **Total Estimated**: ~2-3 SOL

### Ongoing Costs
- **Transaction Fees**: ~0.00025 SOL per transaction
- **Account Rent**: Exempt after initial deposit

## Security Checklist

Before deploying to mainnet:

- [ ] Authority keypair is securely stored
- [ ] Program has been thoroughly tested on devnet
- [ ] All environment variables are set correctly
- [ ] Treasury wallet address is correct
- [ ] FLII token mint address is verified
- [ ] Platform fee percentage is appropriate
- [ ] Backup of all keypairs exists
- [ ] Team has reviewed the contract code

## Troubleshooting

### Common Issues

1. **Insufficient SOL Balance**
   ```bash
   # Check balance
   solana balance --keypair ~/.config/solana/authority-keypair.json --url mainnet-beta
   ```

2. **Wrong Network**
   ```bash
   # Ensure you're on mainnet
   solana config set --url mainnet-beta
   ```

3. **Program Already Exists**
   - If the program ID already exists, you'll need to upgrade instead of deploy
   - Use `solana program upgrade` command

4. **RPC Errors**
   - Try alternative RPC endpoints:
     - https://api.mainnet-beta.solana.com
     - https://solana-api.projectserum.com
     - Your own RPC node

## Support

For deployment support:
- GitHub Issues: https://github.com/fliidotdev/marketplace-contracts
- X/Twitter: @fliidotdev

## Important Notes

⚠️ **MAINNET IS PRODUCTION**: 
- Real money is at stake
- Transactions are irreversible
- Always test on devnet first
- Keep keypairs secure
- Never share private keys

## Next Steps After Deployment

1. **Initialize Marketplace**: Set up treasury and fee structure
2. **Create Frontend**: Build user interface for the marketplace
3. **Add Liquidity**: Ensure FLII token has sufficient liquidity
4. **Marketing**: Announce the marketplace launch
5. **Monitor**: Set up monitoring and alerts for the program

---

**Program Status**: Ready for Mainnet Deployment
**Last Updated**: September 2024
**Version**: 1.0.0
