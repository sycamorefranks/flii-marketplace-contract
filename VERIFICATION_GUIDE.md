# 🔐 Program Verification Guide

## Program Details
- **Program ID**: `3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF`
- **Network**: Solana Mainnet
- **Authority**: `Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY`

## Verification Methods

### Method 1: Solana Verified Builds (Recommended)

1. **Install Docker** (required for reproducible builds)
```bash
# macOS
brew install docker
brew install docker-compose
```

2. **Create verifiable build**
```bash
# Build with verification
anchor build --verifiable

# This creates a deterministic build that can be verified
```

3. **Deploy verification to Anchor Registry**
```bash
# Publish IDL and verification
anchor idl init --filepath target/idl/marketplace.json 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
anchor idl upgrade --filepath target/idl/marketplace.json 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
```

### Method 2: Solscan Verification

1. **Visit Solscan**: https://solscan.io/account/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

2. **Submit Verification Request**:
   - Click "Verify Contract" button
   - Provide GitHub repository: https://github.com/fliidotdev/marketplace-contracts
   - Submit source code

3. **Solscan will**:
   - Build your code
   - Compare bytecode
   - Add verified badge ✅

### Method 3: OtterSec Verification

1. **Visit**: https://github.com/otter-sec/solana-verified-programs

2. **Submit PR** with:
```yaml
- name: "FLII Marketplace"
  program_id: "3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF"
  repo: "https://github.com/fliidotdev/marketplace-contracts"
  commit: "main"
  anchor_version: "0.29.0"
```

### Method 4: Solana FM Verification

1. **Go to**: https://solana.fm/address/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

2. **Click** "Verify Program"

3. **Provide**:
   - GitHub URL
   - Build instructions
   - Anchor version

## Required Files for Verification

### 1. Create `verify.json`
```json
{
  "program_id": "3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF",
  "program_name": "FLII Marketplace",
  "source_code": "https://github.com/fliidotdev/marketplace-contracts",
  "build_command": "anchor build",
  "anchor_version": "0.29.0",
  "solana_version": "1.17.0",
  "rust_version": "1.75.0",
  "commit_hash": "main"
}
```

### 2. Publish Source Code

Create a public GitHub repository with your contract code:

```bash
# Initialize git repo
git init
git add .
git commit -m "FLII Marketplace Smart Contract"

# Create GitHub repo and push
gh repo create fliidotdev/marketplace-contracts --public
git remote add origin https://github.com/fliidotdev/marketplace-contracts.git
git push -u origin main
```

### 3. Add Build Instructions

Create `README.md` with clear build instructions:

```markdown
## Build Instructions

1. Install dependencies:
   - Rust 1.75.0
   - Solana CLI 1.17.0
   - Anchor 0.29.0

2. Build:
   ```bash
   anchor build
   ```

3. Program ID: 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
```

## Verification Status Badges

Once verified, you can add badges to your README:

```markdown
![Verified](https://img.shields.io/badge/Solana-Verified-green)
![Anchor](https://img.shields.io/badge/Anchor-0.29.0-blue)
![Mainnet](https://img.shields.io/badge/Network-Mainnet-orange)
```

## Benefits of Verification

✅ **Trust**: Users can verify the code matches deployment
✅ **Security**: Proves no hidden malicious code
✅ **Transparency**: Source code is public
✅ **Badge**: Get "Verified" badge on explorers
✅ **Credibility**: Required for major integrations

## Quick Verification Checklist

- [ ] Source code on GitHub
- [ ] Clear build instructions
- [ ] Matching Anchor version (0.29.0)
- [ ] Reproducible build setup
- [ ] IDL file available
- [ ] Program deployed to mainnet
- [ ] Authority wallet secured

## Manual Verification Command

Anyone can verify your program matches source by:

```bash
# Clone and build
git clone https://github.com/fliidotdev/marketplace-contracts
cd marketplace-contracts
anchor build

# Compare hash
solana program dump 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF program.so
shasum -a 256 program.so
shasum -a 256 target/deploy/marketplace.so

# Hashes should match
```

## Support

For verification help:
- Anchor Discord: https://discord.gg/anchor
- Solana Tech Discord: https://discord.gg/solana
- OtterSec: https://osec.io

---

**Status**: Ready for verification submission
**Last Updated**: September 2024
