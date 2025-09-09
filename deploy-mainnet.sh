#!/bin/bash

echo "========================================="
echo "FLII Marketplace Mainnet Deployment"
echo "========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Program details
PROGRAM_ID="3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF"
AUTHORITY_WALLET="Fywoy2hHcaz3Qqn5SLF3MyN8q2UAm3fEB8W3XKDfXBcY"
FLII_TOKEN_MINT="BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump"

echo -e "${YELLOW}Program ID:${NC} $PROGRAM_ID"
echo -e "${YELLOW}Authority Wallet:${NC} $AUTHORITY_WALLET"
echo -e "${YELLOW}FLII Token Mint:${NC} $FLII_TOKEN_MINT"
echo ""

# Check if authority keypair exists
if [ ! -f "$HOME/.config/solana/authority-keypair.json" ]; then
    echo -e "${RED}ERROR: Authority keypair not found!${NC}"
    echo "Please place your authority wallet keypair at: $HOME/.config/solana/authority-keypair.json"
    echo "This should be the keypair for wallet: $AUTHORITY_WALLET"
    exit 1
fi

# Step 1: Prepare for mainnet
echo -e "${GREEN}Step 1: Preparing for mainnet deployment...${NC}"

# Replace lib.rs with mainnet version
cp programs/marketplace/src/lib_mainnet.rs programs/marketplace/src/lib.rs

# Step 2: Build the program
echo -e "${GREEN}Step 2: Building the program...${NC}"
anchor build

if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

# Step 3: Check SOL balance
echo -e "${GREEN}Step 3: Checking SOL balance...${NC}"
BALANCE=$(solana balance $AUTHORITY_WALLET --url mainnet-beta 2>/dev/null || echo "0")
echo "Current balance: $BALANCE"

# Estimate deployment cost (approximately 2-3 SOL)
echo -e "${YELLOW}Estimated deployment cost: ~2-3 SOL${NC}"
echo ""

# Step 4: Confirm deployment
read -p "Do you want to proceed with mainnet deployment? (yes/no): " CONFIRM
if [ "$CONFIRM" != "yes" ]; then
    echo "Deployment cancelled."
    exit 0
fi

# Step 5: Deploy to mainnet
echo -e "${GREEN}Step 5: Deploying to mainnet...${NC}"
echo "This may take a few minutes..."

solana program deploy \
    target/deploy/marketplace.so \
    --program-id target/deploy/marketplace-mainnet-keypair.json \
    --keypair $HOME/.config/solana/authority-keypair.json \
    --url mainnet-beta

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}=========================================${NC}"
    echo -e "${GREEN}DEPLOYMENT SUCCESSFUL!${NC}"
    echo -e "${GREEN}=========================================${NC}"
    echo ""
    echo -e "${YELLOW}Program ID:${NC} $PROGRAM_ID"
    echo -e "${YELLOW}Network:${NC} Solana Mainnet"
    echo ""
    echo "View on Explorer:"
    echo "https://explorer.solana.com/address/$PROGRAM_ID"
    echo ""
    echo -e "${GREEN}Next Steps:${NC}"
    echo "1. Initialize the marketplace with your treasury wallet"
    echo "2. Verify the program on Solscan"
    echo "3. Update your frontend with the mainnet program ID"
    echo ""
else
    echo -e "${RED}Deployment failed!${NC}"
    echo "Please check the error messages above."
    exit 1
fi

# Step 6: Initialize the marketplace (optional)
echo ""
read -p "Do you want to initialize the marketplace now? (yes/no): " INIT_CONFIRM
if [ "$INIT_CONFIRM" == "yes" ]; then
    echo -e "${GREEN}Initializing marketplace...${NC}"
    echo "This will set up the marketplace with:"
    echo "- Authority: $AUTHORITY_WALLET"
    echo "- FLII Token: $FLII_TOKEN_MINT"
    echo "- Platform Fee: 2.5% (250 basis points)"
    echo ""
    
    # You would need to use anchor client or custom script to initialize
    echo "Please use the Anchor client or SDK to initialize with these parameters."
fi

echo ""
echo -e "${GREEN}Deployment script completed!${NC}"
