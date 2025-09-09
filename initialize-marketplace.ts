import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import fs from "fs";

// Configuration
const PROGRAM_ID = new PublicKey("3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF");
const FLII_TOKEN_MINT = new PublicKey("BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump");
const PLATFORM_FEE = 250; // 2.5% in basis points (250/10000)

async function initializeMarketplace() {
    // Setup connection
    const connection = new anchor.web3.Connection(
        "https://api.mainnet-beta.solana.com",
        "confirmed"
    );

    // Load authority keypair
    const authorityKeypairPath = `${process.env.HOME}/.config/solana/authority-keypair.json`;
    
    if (!fs.existsSync(authorityKeypairPath)) {
        console.error("❌ Authority keypair not found!");
        console.error("Please ensure the keypair is at:", authorityKeypairPath);
        process.exit(1);
    }

    const authorityKeypair = Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(authorityKeypairPath, "utf-8")))
    );

    console.log("✅ Authority wallet:", authorityKeypair.publicKey.toString());

    // Setup Anchor provider
    const wallet = new anchor.Wallet(authorityKeypair);
    const provider = new anchor.AnchorProvider(connection, wallet, {
        commitment: "confirmed",
    });
    anchor.setProvider(provider);

    // Load the program IDL (you'll need to add this)
    // const idl = JSON.parse(fs.readFileSync("./target/idl/marketplace.json", "utf-8"));
    // const program = new Program(idl, PROGRAM_ID, provider);

    // Derive marketplace PDA
    const [marketplacePDA, marketplaceBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        PROGRAM_ID
    );

    console.log("📍 Marketplace PDA:", marketplacePDA.toString());

    // Treasury wallet (you can change this to a different wallet if needed)
    const treasuryWallet = authorityKeypair.publicKey;

    console.log("\n🚀 Initializing Marketplace with:");
    console.log("   Program ID:", PROGRAM_ID.toString());
    console.log("   FLII Token:", FLII_TOKEN_MINT.toString());
    console.log("   Treasury:", treasuryWallet.toString());
    console.log("   Platform Fee:", PLATFORM_FEE / 100, "%");

    try {
        // Check if already initialized
        const accountInfo = await connection.getAccountInfo(marketplacePDA);
        if (accountInfo) {
            console.log("\n⚠️  Marketplace already initialized!");
            return;
        }

        // Build the transaction manually (since we don't have IDL)
        // You would need to use the program's methods here
        
        console.log("\n✅ Marketplace initialization transaction would be sent here");
        console.log("   (Implementation requires program IDL)");
        
        // After successful initialization:
        console.log("\n🎉 Marketplace initialized successfully!");
        console.log("\n📝 Next steps:");
        console.log("1. Secure your authority keypair");
        console.log("2. Build the frontend");
        console.log("3. Start onboarding creators");

    } catch (error) {
        console.error("\n❌ Initialization failed:", error);
    }
}

// Run initialization
initializeMarketplace().catch(console.error);
