import { PublicKey } from '@solana/web3.js';

export interface Marketplace {
  authority: PublicKey;
  feePercentage: number;
  totalVolume: bigint;
  totalComponents: bigint;
}

export interface Component {
  creator: PublicKey;
  componentId: string;
  price: bigint;
  metadataUri: string;
  isActive: boolean;
  totalSales: bigint;
  createdAt: bigint;
}

export interface Purchase {
  buyer: PublicKey;
  componentId: string;
  price: bigint;
  purchasedAt: bigint;
}

export interface ComponentListedEvent {
  componentId: string;
  creator: PublicKey;
  price: bigint;
}

export interface ComponentPurchasedEvent {
  componentId: string;
  buyer: PublicKey;
  price: bigint;
}
