import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Marketplace } from '../../target/types/marketplace';
import { RevenueShare } from '../../target/types/revenue_share';

async function main() {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load programs
  const marketplaceProgram = anchor.workspace.Marketplace as Program<Marketplace>;
  const revenueShareProgram = anchor.workspace.RevenueShare as Program<RevenueShare>;

  console.log('Deploying programs...');
  console.log('Marketplace Program ID:', marketplaceProgram.programId.toString());
  console.log('Revenue Share Program ID:', revenueShareProgram.programId.toString());

  // Initialize marketplace
  const [marketplacePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('marketplace')],
    marketplaceProgram.programId
  );

  try {
    await marketplaceProgram.methods
      .initialize(3000) // 30% fee
      .accounts({
        marketplace: marketplacePda,
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    console.log('Marketplace initialized successfully');
  } catch (err) {
    console.error('Error initializing marketplace:', err);
  }

  // Initialize revenue pool
  const [poolPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('revenue_pool')],
    revenueShareProgram.programId
  );

  try {
    await revenueShareProgram.methods
      .initializePool(7000, 3000) // 70% creator, 30% platform
      .accounts({
        pool: poolPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    console.log('Revenue pool initialized successfully');
  } catch (err) {
    console.error('Error initializing revenue pool:', err);
  }
}

main().then(
  () => process.exit(0),
).catch(err => {
  console.error(err);
  process.exit(1);
});
