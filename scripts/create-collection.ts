/**
 * Create Collection NFT for Social-Fi Usernames
 * This is infrastructure setup - run once by platform admin
 * 
 * Usage:
 *   pnpm tsx scripts/create-collection.ts
 *   make create-collection
 */

import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import * as fs from 'fs';

// Load config
const network = process.env.ANCHOR_PROVIDER_URL || 'https://api.devnet.solana.com';
const isDevnet = network.includes('devnet');

async function main() {
  console.log('🏛️  Creating Social-Fi Collection NFT\n');
  console.log(`Network: ${isDevnet ? 'devnet' : 'mainnet-beta'}`);
  console.log(`RPC: ${network}\n`);

  // Connect
  const connection = new Connection(network, 'confirmed');

  // Load wallet
  const keypairPath = process.env.ANCHOR_WALLET || 
    `${process.env.HOME}/.config/solana/id.json`;
  
  if (!fs.existsSync(keypairPath)) {
    console.error('❌ Keypair not found:', keypairPath);
    console.log('Set ANCHOR_WALLET env var or create keypair:');
    console.log('  solana-keygen new\n');
    process.exit(1);
  }

  const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
  const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));
  const authority = payer.publicKey;

  console.log('👤 Authority:', authority.toString());

  // Check balance
  const balance = await connection.getBalance(authority);
  console.log(`💰 Balance: ${(balance / 1e9).toFixed(4)} SOL\n`);

  if (balance < 0.01 * 1e9) {
    console.error('❌ Insufficient balance. Need at least 0.01 SOL');
    if (isDevnet) {
      console.log('Request airdrop: solana airdrop 1\n');
    }
    process.exit(1);
  }

  console.log('⚙️  Creating collection mint...\n');

  // Create collection mint (NFT with 0 decimals)
  const collectionMint = await createMint(
    connection,
    payer,
    authority, // mint authority
    authority, // freeze authority
    0 // 0 decimals = NFT
  );

  console.log('✅ Collection mint created!\n');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('📋 COLLECTION CONFIGURATION');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  console.log(`Collection Mint:  ${collectionMint.toString()}`);
  console.log(`Authority:        ${authority.toString()}\n`);

  // Save to .env
  const envPath = `${__dirname}/../.env`;
  let envContent = '';
  
  if (fs.existsSync(envPath)) {
    envContent = fs.readFileSync(envPath, 'utf-8');
  }

  // Update or add collection mint
  const hasCollectionMint = /COLLECTION_MINT=/m.test(envContent);
  const hasCollectionAuthority = /COLLECTION_AUTHORITY=/m.test(envContent);

  if (hasCollectionMint) {
    envContent = envContent.replace(
      /COLLECTION_MINT=.*/,
      `COLLECTION_MINT=${collectionMint.toString()}`
    );
  } else {
    envContent += `\nCOLLECTION_MINT=${collectionMint.toString()}`;
  }

  if (hasCollectionAuthority) {
    envContent = envContent.replace(
      /COLLECTION_AUTHORITY=.*/,
      `COLLECTION_AUTHORITY=${authority.toString()}`
    );
  } else {
    envContent += `\nCOLLECTION_AUTHORITY=${authority.toString()}`;
  }

  fs.writeFileSync(envPath, envContent.trim() + '\n');
  console.log('✅ Saved to .env\n');

  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('📝 NEXT STEPS');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  
  console.log('1. Verify on Solscan:');
  const cluster = isDevnet ? '?cluster=devnet' : '';
  console.log(`   https://solscan.io/token/${collectionMint.toString()}${cluster}\n`);

  console.log('2. Update Frontend .env:');
  console.log(`   VITE_COLLECTION_MINT=${collectionMint.toString()}`);
  console.log(`   VITE_COLLECTION_AUTHORITY=${authority.toString()}\n`);

  console.log('3. (Optional) Create token account and mint 1 NFT:');
  console.log(`   spl-token create-account ${collectionMint.toString()}`);
  console.log(`   spl-token mint ${collectionMint.toString()} 1\n`);

  console.log('4. Deploy contract (if not already):');
  console.log('   make deploy-devnet\n');

  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error('❌ Error:', error);
    process.exit(1);
  });
