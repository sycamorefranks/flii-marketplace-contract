#!/usr/bin/env node

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
const crypto = require('crypto');

// Configuration
const MARKETPLACE_PROGRAM_ID = new PublicKey("3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF");
const FLII_TOKEN_MINT = new PublicKey("BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump");
const PLATFORM_FEE = 250; // 2.5%
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");

// Generate Anchor discriminator for "initialize" instruction
function getDiscriminator(instructionName) {
    const hash = crypto.createHash('sha256');
    hash.update(`global:${instructionName}`);
    const digest = hash.digest();
    return digest.slice(0, 8);
}

async function initializeMarketplace() {
    console.log("🚀 FLII Marketplace Initialization (Fixed)");
    console.log("==========================================");
    
    try {
        // Load authority keypair
        const authorityKeypairPath = `${process.env.HOME}/.config/solana/authority-keypair.json`;
        const authorityKeypairData = JSON.parse(fs.readFileSync(authorityKeypairPath, 'utf-8'));
        const authorityKeypair = Keypair.fromSecretKey(new Uint8Array(authorityKeypairData));
        
        console.log("✅ Authority:", authorityKeypair.publicKey.toString());
        
        // Check balance
        const balance = await connection.getBalance(authorityKeypair.publicKey);
        console.log("💰 Balance:", balance / 1e9, "SOL");
        
        // Derive PDA
        const [marketplacePDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("marketplace")],
            MARKETPLACE_PROGRAM_ID
        );
        
        console.log("📍 Marketplace PDA:", marketplacePDA.toString());
        
        // Check if initialized
        const accountInfo = await connection.getAccountInfo(marketplacePDA);
        if (accountInfo && accountInfo.data.length > 0) {
            console.log("⚠️  Marketplace may already be initialized!");
            console.log("    Data length:", accountInfo.data.length);
            
            // Check if it's actually initialized by looking at the data
            if (accountInfo.data.length >= 58) { // Expected size of initialized marketplace
                console.log("\n✅ Marketplace appears to be initialized!");
                console.log("🎉 You can now start listing and trading components!");
                return;
            }
        }
        
        // Treasury (using authority for now)
        const treasuryWallet = authorityKeypair.publicKey;
        
        console.log("\n📋 Parameters:");
        console.log("   FLII Token:", FLII_TOKEN_MINT.toString());
        console.log("   Treasury:", treasuryWallet.toString());
        console.log("   Fee:", PLATFORM_FEE / 100 + "%");
        
        // Build instruction with Anchor discriminator
        const discriminator = getDiscriminator("initialize");
        console.log("   Discriminator:", discriminator.toString('hex'));
        
        // Create instruction data: [discriminator (8 bytes), fee_percentage (2 bytes, LE)]
        const instructionData = Buffer.alloc(10);
        discriminator.copy(instructionData, 0);
        instructionData.writeUInt16LE(PLATFORM_FEE, 8);
        
        // Create instruction
        const initInstruction = new TransactionInstruction({
            programId: MARKETPLACE_PROGRAM_ID,
            keys: [
                { pubkey: marketplacePDA, isSigner: false, isWritable: true },
                { pubkey: FLII_TOKEN_MINT, isSigner: false, isWritable: false },
                { pubkey: treasuryWallet, isSigner: false, isWritable: false },
                { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            ],
            data: instructionData
        });
        
        // Send transaction
        const transaction = new Transaction().add(initInstruction);
        
        console.log("\n📤 Sending transaction...");
        
        try {
            const signature = await sendAndConfirmTransaction(
                connection,
                transaction,
                [authorityKeypair],
                {
                    commitment: 'confirmed',
                    skipPreflight: false
                }
            );
            
            console.log("\n✅ SUCCESS!");
            console.log("🎉 Transaction:", signature);
            console.log("🔗 View: https://solscan.io/tx/" + signature);
            
            // Verify
            const newInfo = await connection.getAccountInfo(marketplacePDA);
            if (newInfo) {
                console.log("\n📊 Marketplace initialized!");
                console.log("   Size:", newInfo.data.length, "bytes");
                console.log("   Owner:", newInfo.owner.toString());
            }
            
            console.log("\n🎊 Your marketplace is LIVE!");
            console.log("\n📝 Next steps:");
            console.log("1. Add marketplace UI to your app");
            console.log("2. List components");
            console.log("3. Start earning FLII!");
            
        } catch (txError) {
            console.error("\n❌ Transaction error:", txError.message);
            
            if (txError.logs) {
                console.error("\nLogs:");
                txError.logs.forEach(log => console.error("  ", log));
            }
            
            // If it's already initialized, that's actually OK!
            if (txError.message.includes("already in use") || 
                txError.message.includes("custom program error: 0x0")) {
                console.log("\n✅ Good news! The marketplace is already initialized!");
                console.log("🎉 You can start using it immediately!");
            }
        }
        
    } catch (error) {
        console.error("\n❌ Error:", error.message);
    }
}

// Run
console.log("Starting initialization...\n");
initializeMarketplace().then(() => {
    console.log("\n✨ Done!");
}).catch(console.error);
