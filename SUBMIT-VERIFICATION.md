# 🔐 Program Verification Submission Guide

## Your Program Details
- **Program ID**: `3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF`
- **GitHub Repo**: https://github.com/fliidotdev/flii-marketplace-contract
- **Status**: Ready for verification

## 📋 Step-by-Step Verification Process

### 1️⃣ **Solscan Verification** (Easiest & Fastest)

1. **Visit your program on Solscan:**
   https://solscan.io/account/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

2. **Click "Update Info" button** (top right)

3. **Fill out the form:**
   ```
   Program Name: FLII Marketplace
   Description: Component marketplace for web developers using FLII tokens
   Website: https://yes-no.fun
   GitHub: https://github.com/fliidotdev/flii-marketplace-contract
   Twitter: @yesnodotfun
   ```

4. **Submit verification request**
   - They will review within 24-48 hours
   - You'll get a green verified badge ✅

### 2️⃣ **OtterSec Verification** (Most Trusted)

1. **Go to:** https://github.com/otter-sec/solana-verified-programs

2. **Click "Create Pull Request"**

3. **Add your program to `verified-programs.json`:**
   ```json
   {
     "name": "FLII Marketplace",
     "program_id": "3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF",
     "repo": "https://github.com/fliidotdev/flii-marketplace-contract",
     "commit": "main",
     "anchor_version": "0.29.0",
     "verified_date": "2024-09-09",
     "network": "mainnet"
   }
   ```

4. **Submit PR and wait for review**

### 3️⃣ **Anchor Verified Build**

Run these commands to create a verified build:

```bash
# Install Docker if not installed
brew install docker

# Build verifiable
cd /Users/dizrealcastillo/flii-dev/marketplace-contracts
anchor build --verifiable

# Upload IDL
anchor idl init 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF \
  --filepath target/idl/marketplace.json \
  --provider.cluster mainnet
```

### 4️⃣ **Solana FM Verification**

1. **Visit:** https://solana.fm/address/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

2. **Click "Verify Program"**

3. **Submit:**
   - GitHub URL: https://github.com/fliidotdev/flii-marketplace-contract
   - Build Command: `anchor build`
   - Anchor Version: 0.29.0

### 5️⃣ **Submit to Solana Program Registry**

Email to: registry@solana.com

```
Subject: Program Verification Request - FLII Marketplace

Program ID: 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF
Program Name: FLII Marketplace
GitHub: https://github.com/fliidotdev/flii-marketplace-contract
Description: Component marketplace for web developers using FLII tokens
Network: Mainnet
Authority: Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY

Build Instructions:
- Anchor Version: 0.29.0
- Command: anchor build

Please verify this program for the Solana ecosystem.

Thank you,
FLII Team
```

## 📝 Copy-Paste Templates

### For Solscan Form:
```
Name: FLII Marketplace
Type: DeFi/Marketplace
Description: A decentralized marketplace for web developers to buy and sell components using FLII tokens. Features include 2.5% platform fees, 2% creator rewards, and IPFS/Arweave storage integration.
Website: https://yes-no.fun
Twitter: https://twitter.com/yesnodotfun
GitHub: https://github.com/fliidotdev/flii-marketplace-contract
Docs: https://github.com/fliidotdev/flii-marketplace-contract/blob/main/README.md
```

### For GitHub README badges (after verification):
```markdown
![Verified](https://img.shields.io/badge/Solana-Verified-green)
![Anchor](https://img.shields.io/badge/Anchor-0.29.0-blue)
![Mainnet](https://img.shields.io/badge/Network-Mainnet-orange)
![OtterSec](https://img.shields.io/badge/OtterSec-Verified-green)
```

## ⏱️ Expected Timeline

- **Solscan**: 24-48 hours
- **OtterSec**: 3-5 days
- **Anchor Registry**: Immediate after IDL upload
- **Solana FM**: 2-3 days

## ✅ Verification Checklist

Before submitting:
- [ ] GitHub repository is public
- [ ] README has build instructions
- [ ] verify.json file is present
- [ ] Source code matches deployed program
- [ ] Anchor.toml has correct program ID

## 🎯 Priority Order

1. **Submit to Solscan first** (fastest, most visible)
2. **Upload IDL to Anchor** (enables better integration)
3. **Submit to OtterSec** (most trusted verification)
4. **Add to Solana FM** (additional visibility)

## 🔗 Quick Links

- **Your Program**: [View on Solscan](https://solscan.io/account/3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF)
- **Deployment TX**: [View Transaction](https://solscan.io/tx/33Chp2kh276AjkA2HmnURUsup84EjRVq5DFxCWgoyxpHYVMN41wRP7YnHtfo89MwzDP3adNSj3kHUPCF65CpZFMq)
- **GitHub Repo**: [View Code](https://github.com/fliidotdev/flii-marketplace-contract)

## 📞 Support

If you need help with verification:
- Solscan Support: support@solscan.io
- OtterSec: hello@osec.io
- Anchor Discord: https://discord.gg/anchor

---

**Your program is ready for verification! Start with Solscan for the quickest results.**
