#!/usr/bin/env node

/**
 * FLII Marketplace Initialization Script
 * This will initialize your marketplace on Solana Mainnet
 */

const { 
    Connection, 
    PublicKey, 
    Keypair, 
    Transaction,
    TransactionInstruction,
    SystemProgram,
    sendAndConfirmTransaction
} = require('@solana/web3.js');
const fs = require('fs');
const bs58 = require('bs58');

// ====== CONFIGURATION ======
const MARKETPLACE_PROGRAM_ID = new PublicKey("3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF");
const FLII_TOKEN_MINT = new PublicKey("BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump");
const PLATFORM_FEE = 250; // 2.5% in basis points (250 / 10000 = 0.025)

// Connection to Solana Mainnet
const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");

async function initializeMarketplace() {
    console.log("🚀 FLII Marketplace Initialization");
    console.log("=====================================");
    
    try {
        // Load authority keypair
        const authorityKeypairPath = `${process.env.HOME}/.config/solana/authority-keypair.json`;
        
        if (!fs.existsSync(authorityKeypairPath)) {
            console.error("❌ Authority keypair not found at:", authorityKeypairPath);
            process.exit(1);
        }
        
        const authorityKeypairData = JSON.parse(fs.readFileSync(authorityKeypairPath, 'utf-8'));
        const authorityKeypair = Keypair.fromSecretKey(new Uint8Array(authorityKeypairData));
        
        console.log("✅ Authority Wallet:", authorityKeypair.publicKey.toString());
        
        // Check SOL balance
        const balance = await connection.getBalance(authorityKeypair.publicKey);
        console.log("💰 SOL Balance:", balance / 1e9, "SOL");
        
        if (balance < 0.01 * 1e9) {
            console.error("❌ Insufficient SOL balance for initialization");
            process.exit(1);
        }
        
        // Derive marketplace PDA
        const [marketplacePDA, bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("marketplace")],
            MARKETPLACE_PROGRAM_ID
        );
        
        console.log("📍 Marketplace PDA:", marketplacePDA.toString());
        console.log("📍 Bump:", bump);
        
        // Check if already initialized
        const accountInfo = await connection.getAccountInfo(marketplacePDA);
        if (accountInfo && accountInfo.data.length > 0) {
            console.log("⚠️  Marketplace appears to be already initialized!");
            console.log("    Account data length:", accountInfo.data.length);
            
            // Try to decode the data to check initialization status
            if (accountInfo.data.length >= 8) {
                const discriminator = accountInfo.data.slice(0, 8);
                console.log("    Discriminator:", discriminator.toString('hex'));
                console.log("\n⚠️  Marketplace may already be initialized. Proceed with caution.");
                
                // Ask for confirmation
                console.log("\nDo you want to continue anyway? This might fail if already initialized.");
                console.log("Press Ctrl+C to cancel, or wait 5 seconds to continue...");
                await new Promise(resolve => setTimeout(resolve, 5000));
            }
        } else {
            console.log("✅ Marketplace not yet initialized - ready to proceed!");
        }
        
        // Treasury wallet (using authority as treasury for now)
        const treasuryWallet = authorityKeypair.publicKey;
        
        console.log("\n📋 Initialization Parameters:");
        console.log("   Program ID:", MARKETPLACE_PROGRAM_ID.toString());
        console.log("   FLII Token:", FLII_TOKEN_MINT.toString());
        console.log("   Treasury:", treasuryWallet.toString());
        console.log("   Platform Fee:", PLATFORM_FEE / 100 + "%");
        
        // Build initialization instruction
        console.log("\n🔨 Building initialization transaction...");
        
        // Create the instruction data
        // Instruction format: [instruction_index (1 byte), fee_percentage (2 bytes, little endian)]
        const instructionData = Buffer.alloc(3);
        instructionData[0] = 0; // Initialize instruction index
        instructionData.writeUInt16LE(PLATFORM_FEE, 1); // Fee percentage
        
        // Create the initialization instruction
        const initInstruction = new TransactionInstruction({
            programId: MARKETPLACE_PROGRAM_ID,
            keys: [
                { pubkey: marketplacePDA, isSigner: false, isWritable: true },
                { pubkey: FLII_TOKEN_MINT, isSigner: false, isWritable: false },
                { pubkey: treasuryWallet, isSigner: false, isWritable: false },
                { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
                { 
                    pubkey: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"), 
                    isSigner: false, 
                    isWritable: false 
                }, // Token Program
            ],
            data: instructionData
        });
        
        // Create and send transaction
        const transaction = new Transaction().add(initInstruction);
        
        console.log("\n📤 Sending initialization transaction...");
        
        try {
            const signature = await sendAndConfirmTransaction(
                connection,
                transaction,
                [authorityKeypair],
                {
                    commitment: 'confirmed',
                    preflightCommitment: 'confirmed',
                    skipPreflight: false
                }
            );
            
            console.log("\n✅ MARKETPLACE INITIALIZED SUCCESSFULLY!");
            console.log("🎉 Transaction Signature:", signature);
            console.log("🔗 View on Solscan: https://solscan.io/tx/" + signature);
            
            // Verify initialization
            const newAccountInfo = await connection.getAccountInfo(marketplacePDA);
            if (newAccountInfo) {
                console.log("\n📊 Marketplace Account Info:");
                console.log("   Data Length:", newAccountInfo.data.length, "bytes");
                console.log("   Owner:", newAccountInfo.owner.toString());
                console.log("   Lamports:", newAccountInfo.lamports / 1e9, "SOL");
            }
            
            console.log("\n🎊 Your marketplace is now live and ready to accept component listings!");
            console.log("\n📝 Next Steps:");
            console.log("1. ✅ Add marketplace UI to your web app");
            console.log("2. ✅ List your first component");
            console.log("3. ✅ Share with the community");
            console.log("4. ✅ Start earning FLII tokens!");
            
        } catch (error) {
            console.error("\n❌ Transaction failed:", error.message);
            
            if (error.logs) {
                console.error("\n📜 Transaction logs:");
                error.logs.forEach(log => console.error("   ", log));
            }
            
            // Provide helpful error messages
            if (error.message.includes("already in use")) {
                console.error("\n⚠️  The marketplace appears to be already initialized.");
                console.error("    You can start using it immediately!");
            } else if (error.message.includes("insufficient funds")) {
                console.error("\n⚠️  Insufficient SOL for transaction fees.");
                console.error("    Please add more SOL to your wallet.");
            } else if (error.message.includes("InvalidAccountData")) {
                console.error("\n⚠️  The account data structure might not match.");
                console.error("    This could mean the program needs different initialization parameters.");
            }
        }
        
    } catch (error) {
        console.error("\n❌ Initialization error:", error.message);
        console.error(error.stack);
    }
}

// Run the initialization
console.log("Starting marketplace initialization...\n");
initializeMarketplace().then(() => {
    console.log("\n✨ Script completed!");
}).catch(error => {
    console.error("\n❌ Fatal error:", error);
    process.exit(1);
});
