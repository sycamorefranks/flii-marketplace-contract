// Frontend Integration for FLII Marketplace
// Add this to your existing web app

import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from '@solana/spl-token';

// Constants
const MARKETPLACE_PROGRAM_ID = new PublicKey("3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF");
const FLII_TOKEN_MINT = new PublicKey("BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump");

// Initialize connection (you probably already have this)
const connection = new Connection("https://api.mainnet-beta.solana.com");

// 1. LIST A COMPONENT
export async function listComponent(wallet, componentData) {
    const { 
        componentId,    // unique ID for component
        price,          // price in FLII tokens (with decimals)
        metadataUri     // IPFS/Arweave URI for component code
    } = componentData;

    // Create listing transaction
    const tx = new Transaction();
    
    // Add listing instruction (simplified - needs actual instruction data)
    const listingInstruction = {
        programId: MARKETPLACE_PROGRAM_ID,
        keys: [
            { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
            // Add other required accounts
        ],
        data: Buffer.from([
            // Instruction data for list_component
        ])
    };
    
    tx.add(listingInstruction);
    
    // Send transaction
    const signature = await wallet.sendTransaction(tx, connection);
    await connection.confirmTransaction(signature);
    
    return signature;
}

// 2. PURCHASE A COMPONENT
export async function purchaseComponent(wallet, componentId) {
    // Get user's FLII token account
    const buyerTokenAccount = await getAssociatedTokenAddress(
        FLII_TOKEN_MINT,
        wallet.publicKey
    );
    
    // Create purchase transaction
    const tx = new Transaction();
    
    // Add purchase instruction (simplified)
    const purchaseInstruction = {
        programId: MARKETPLACE_PROGRAM_ID,
        keys: [
            { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
            { pubkey: buyerTokenAccount, isSigner: false, isWritable: true },
            // Add other required accounts
        ],
        data: Buffer.from([
            // Instruction data for purchase_component
        ])
    };
    
    tx.add(purchaseInstruction);
    
    // Send transaction
    const signature = await wallet.sendTransaction(tx, connection);
    await connection.confirmTransaction(signature);
    
    return signature;
}

// 3. GET ALL COMPONENTS (Read from chain)
export async function getAllComponents() {
    // Fetch all component accounts
    const components = await connection.getProgramAccounts(
        MARKETPLACE_PROGRAM_ID,
        {
            filters: [
                // Add filters to get only component accounts
                { dataSize: 293 }, // Component account size
            ]
        }
    );
    
    // Parse and return component data
    return components.map(({ pubkey, account }) => {
        // Parse account data
        const data = account.data;
        return {
            pubkey: pubkey.toString(),
            // Parse component fields from data
            creator: new PublicKey(data.slice(8, 40)).toString(),
            price: data.readBigUInt64LE(40),
            // ... other fields
        };
    });
}

// 4. GET USER'S COMPONENTS
export async function getUserComponents(userWallet) {
    const components = await connection.getProgramAccounts(
        MARKETPLACE_PROGRAM_ID,
        {
            filters: [
                { dataSize: 293 },
                {
                    memcmp: {
                        offset: 8, // Creator field offset
                        bytes: userWallet.toBase58()
                    }
                }
            ]
        }
    );
    
    return components;
}

// 5. UPLOAD COMPONENT TO IPFS/ARWEAVE
export async function uploadComponent(componentCode, metadata) {
    // Option A: Upload to Arweave
    // const arweave = Arweave.init({...});
    // const transaction = await arweave.createTransaction({
    //     data: JSON.stringify({ code: componentCode, metadata })
    // });
    // await arweave.transactions.sign(transaction);
    // await arweave.transactions.post(transaction);
    // return `https://arweave.net/${transaction.id}`;
    
    // Option B: Upload to IPFS via Pinata
    const formData = new FormData();
    formData.append('file', new Blob([JSON.stringify({
        code: componentCode,
        metadata: metadata
    })], { type: 'application/json' }));
    
    const response = await fetch('https://api.pinata.cloud/pinning/pinFileToIPFS', {
        method: 'POST',
        headers: {
            'Authorization': `Bearer YOUR_PINATA_JWT`
        },
        body: formData
    });
    
    const { IpfsHash } = await response.json();
    return `ipfs://${IpfsHash}`;
}

// 6. INITIALIZE MARKETPLACE (One-time admin function)
export async function initializeMarketplace(authorityWallet) {
    const [marketplacePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        MARKETPLACE_PROGRAM_ID
    );
    
    const tx = new Transaction();
    
    // Add initialize instruction
    const initInstruction = {
        programId: MARKETPLACE_PROGRAM_ID,
        keys: [
            { pubkey: marketplacePDA, isSigner: false, isWritable: true },
            { pubkey: FLII_TOKEN_MINT, isSigner: false, isWritable: false },
            { pubkey: authorityWallet.publicKey, isSigner: true, isWritable: true },
            { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        ],
        data: Buffer.from([
            0, // Initialize instruction index
            250, 0, // Fee percentage (2.5% = 250 basis points)
        ])
    };
    
    tx.add(initInstruction);
    
    const signature = await authorityWallet.sendTransaction(tx, connection);
    await connection.confirmTransaction(signature);
    
    console.log("Marketplace initialized!", signature);
    return signature;
}

// Example React Component Integration
export function MarketplaceComponent() {
    const { wallet } = useWallet(); // Your existing wallet hook
    
    const handleListComponent = async () => {
        // Get component code from user
        const componentCode = document.getElementById('component-code').value;
        
        // Upload to IPFS/Arweave
        const metadataUri = await uploadComponent(componentCode, {
            name: "My Component",
            description: "A great component",
            version: "1.0.0"
        });
        
        // List on marketplace
        await listComponent(wallet, {
            componentId: generateUniqueId(),
            price: 100 * 1e9, // 100 FLII (adjust decimals)
            metadataUri
        });
    };
    
    const handlePurchase = async (componentId) => {
        await purchaseComponent(wallet, componentId);
    };
    
    return (
        <div>
            {/* Your UI here */}
        </div>
    );
}
