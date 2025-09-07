import { Program, AnchorProvider, Idl, setProvider } from '@coral-xyz/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Marketplace } from './types';

export class MarketplaceClient {
  program: Program<Marketplace>;
  provider: AnchorProvider;

  constructor(
    connection: Connection,
    wallet: any,
    programId: PublicKey
  ) {
    this.provider = new AnchorProvider(
      connection,
      wallet,
      { commitment: 'confirmed' }
    );
    setProvider(this.provider);
    
    // Load IDL and create program
    // this.program = new Program(idl, programId, this.provider);
  }

  async initialize(feePercentage: number) {
    const [marketplacePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('marketplace')],
      this.program.programId
    );

    return await this.program.methods
      .initialize(feePercentage)
      .accounts({
        marketplace: marketplacePda,
        authority: this.provider.wallet.publicKey,
      })
      .rpc();
  }

  async listComponent(
    componentId: string,
    price: bigint,
    metadataUri: string
  ) {
    const [componentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('component'), Buffer.from(componentId)],
      this.program.programId
    );

    const [marketplacePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('marketplace')],
      this.program.programId
    );

    return await this.program.methods
      .listComponent(componentId, price, metadataUri)
      .accounts({
        component: componentPda,
        marketplace: marketplacePda,
        creator: this.provider.wallet.publicKey,
      })
      .rpc();
  }

  async purchaseComponent(
    componentId: string,
    component: PublicKey,
    creatorTokenAccount: PublicKey,
    marketplaceTokenAccount: PublicKey
  ) {
    const [purchasePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('purchase'),
        this.provider.wallet.publicKey.toBuffer(),
        Buffer.from(componentId)
      ],
      this.program.programId
    );

    const [marketplacePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('marketplace')],
      this.program.programId
    );

    return await this.program.methods
      .purchaseComponent()
      .accounts({
        component,
        marketplace: marketplacePda,
        purchase: purchasePda,
        buyer: this.provider.wallet.publicKey,
        creatorTokenAccount,
        marketplaceTokenAccount,
      })
      .rpc();
  }
}
