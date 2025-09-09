#!/bin/bash

# FLII Marketplace Verification Submission Script
# Program ID: 3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF

echo "======================================"
echo "FLII Marketplace Verification Process"
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Program details
PROGRAM_ID="3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF"
GITHUB_REPO="https://github.com/sycamorefranks/flii-marketplace-contract"
WEBSITE="https://yes-no.fun"
TWITTER="@yesnodotfun"

echo -e "${BLUE}Program Details:${NC}"
echo "• Program ID: $PROGRAM_ID"
echo "• GitHub: $GITHUB_REPO"
echo "• Website: $WEBSITE"
echo "• Twitter: $TWITTER"
echo ""

# Step 1: Open Solscan for manual verification
echo -e "${YELLOW}Step 1: Solscan Verification (Manual)${NC}"
echo "Opening Solscan in your browser..."
echo "Please click 'Update Info' and submit the following:"
echo ""
echo "Program Name: FLII Marketplace"
echo "Description: Component marketplace for web developers using FLII tokens"
echo "Website: $WEBSITE"
echo "GitHub: $GITHUB_REPO"
echo "Twitter: $TWITTER"
echo ""
open "https://solscan.io/account/$PROGRAM_ID"
echo "Press Enter when you've submitted the Solscan verification..."
read -r

# Step 2: OtterSec GitHub PR
echo -e "${YELLOW}Step 2: OtterSec Verification${NC}"
echo "The following JSON needs to be added to OtterSec's verified-programs.json:"
echo ""
cat ottersec-submission.json
echo ""
echo "Opening OtterSec repository..."
open "https://github.com/otter-sec/solana-verified-programs"
echo ""
echo "Steps:"
echo "1. Fork the repository"
echo "2. Add the above JSON to verified-programs.json"
echo "3. Create a pull request"
echo ""
echo "Press Enter when you've created the PR..."
read -r

# Step 3: Solana FM Verification
echo -e "${YELLOW}Step 3: Solana FM Verification${NC}"
echo "Opening Solana FM..."
open "https://solana.fm/address/$PROGRAM_ID"
echo ""
echo "Click 'Verify Program' and submit:"
echo "• GitHub URL: $GITHUB_REPO"
echo "• Build Command: anchor build"
echo "• Anchor Version: 0.29.0"
echo ""
echo "Press Enter when you've submitted to Solana FM..."
read -r

# Step 4: Create verification tracking file
echo -e "${YELLOW}Step 4: Creating verification tracking file...${NC}"
cat > verification-status.md << EOF
# Verification Status

## Program: FLII Marketplace
**Program ID:** \`$PROGRAM_ID\`

## Verification Platforms

| Platform | Status | Date Submitted | Expected Completion |
|----------|--------|----------------|-------------------|
| Solscan | ⏳ Pending | $(date +%Y-%m-%d) | 24-48 hours |
| OtterSec | ⏳ Pending | $(date +%Y-%m-%d) | 3-5 days |
| Solana FM | ⏳ Pending | $(date +%Y-%m-%d) | 2-3 days |
| Anchor IDL | ❌ Not Started | - | Immediate |

## Links
- [View on Solscan](https://solscan.io/account/$PROGRAM_ID)
- [View on Solana FM](https://solana.fm/address/$PROGRAM_ID)
- [GitHub Repository]($GITHUB_REPO)
- [Deployment Transaction](https://solscan.io/tx/33Chp2kh276AjkA2HmnURUsup84EjRVq5DFxCWgoyxpHYVMN41wRP7YnHtfo89MwzDP3adNSj3kHUPCF65CpZFMq)

## Next Steps
1. Monitor Solscan for verification approval
2. Check OtterSec PR status
3. Follow up with Solana FM if needed
4. Upload IDL to Anchor registry when possible

Last Updated: $(date)
EOF

echo -e "${GREEN}✅ Verification tracking file created!${NC}"
echo ""

# Step 5: Commit and push updates
echo -e "${YELLOW}Step 5: Pushing verification files to GitHub...${NC}"
git add ottersec-submission.json verification-status.md SUBMIT-VERIFICATION.md
git commit -m "Add verification submission files and status tracking"
git push origin main

echo ""
echo -e "${GREEN}======================================"
echo "Verification Process Complete!"
echo "======================================"
echo ""
echo "Summary:"
echo "✅ Code pushed to GitHub"
echo "✅ Solscan submission instructions provided"
echo "✅ OtterSec PR instructions provided"
echo "✅ Solana FM submission instructions provided"
echo "✅ Verification tracking file created"
echo ""
echo "Monitor verification-status.md for updates"
echo "Most verifications complete within 2-5 days"
echo ""
echo "For support:"
echo "• Solscan: support@solscan.io"
echo "• OtterSec: hello@osec.io"
echo "• Anchor Discord: https://discord.gg/anchor"
${NC}
