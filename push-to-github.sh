#!/bin/bash

echo "🚀 Pushing FLII Marketplace Contract to GitHub for Verification"
echo "=============================================================="

# Create GitHub repository (requires GitHub CLI)
echo "Creating GitHub repository..."
gh repo create fliidotdev/flii-marketplace-contract --public --description "FLII Marketplace Smart Contract - Component marketplace using FLII tokens" --homepage "https://yes-no.fun" || echo "Repository may already exist"

# Add remote
git remote add origin https://github.com/fliidotdev/flii-marketplace-contract.git 2>/dev/null || git remote set-url origin https://github.com/fliidotdev/flii-marketplace-contract.git

# Push to GitHub
echo "Pushing to GitHub..."
git branch -M main
git push -u origin main --force

echo ""
echo "✅ Repository pushed to GitHub!"
echo "📍 Repository URL: https://github.com/fliidotdev/flii-marketplace-contract"
echo ""
echo "Next steps for verification:"
echo "1. Visit the repository to confirm it's public"
echo "2. Submit for verification (see instructions below)"
