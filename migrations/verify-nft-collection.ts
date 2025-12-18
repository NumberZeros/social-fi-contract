/**
 * Verify existing username NFTs into collection
 * Use this to migrate old NFTs that were minted before collection was created
 * 
 * Usage:
 *   pnpm tsx scripts/verify-nft-collection.ts <nft_mint_address>
 */

import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as fs from 'fs';

// Load from .env
const network = process.env.ANCHOR_PROVIDER_URL || 'https://api.devnet.solana.com';
const isDevnet = network.includes('devnet');

async function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0) {
    console.error('❌ Usage: pnpm tsx scripts/verify-nft-collection.ts <nft_mint_address>');
    console.log('\nExample:');
    console.log('  pnpm tsx scripts/verify-nft-collection.ts 5xK3m8...abc123\n');
    process.exit(1);
  }

  const nftMint = args[0];

  console.log('🔗 Verifying NFT into Collection\n');
  console.log(`Network: ${isDevnet ? 'devnet' : 'mainnet-beta'}`);
  console.log(`NFT Mint: ${nftMint}\n`);

  // Load collection config
  const envPath = `${__dirname}/../.env`;
  if (!fs.existsSync(envPath)) {
    console.error('❌ .env not found. Run make create-collection first\n');
    process.exit(1);
  }

  const envContent = fs.readFileSync(envPath, 'utf-8');
  const collectionMintMatch = envContent.match(/COLLECTION_MINT=(.+)/);
  const collectionAuthorityMatch = envContent.match(/COLLECTION_AUTHORITY=(.+)/);

  if (!collectionMintMatch || !collectionAuthorityMatch) {
    console.error('❌ Collection not configured. Run make create-collection first\n');
    process.exit(1);
  }

  const collectionMint = collectionMintMatch[1].trim();
  const collectionAuthority = collectionAuthorityMatch[1].trim();

  console.log(`Collection: ${collectionMint}`);
  console.log(`Authority:  ${collectionAuthority}\n`);

  // Connect
  const connection = new Connection(network, 'confirmed');

  // Load wallet
  const keypairPath = process.env.ANCHOR_WALLET || 
    `${process.env.HOME}/.config/solana/id.json`;
  
  if (!fs.existsSync(keypairPath)) {
    console.error('❌ Keypair not found:', keypairPath);
    process.exit(1);
  }

  const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
  const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));

  if (payer.publicKey.toString() !== collectionAuthority) {
    console.error('❌ Wallet mismatch!');
    console.log(`Expected: ${collectionAuthority}`);
    console.log(`Got:      ${payer.publicKey.toString()}\n`);
    process.exit(1);
  }

  console.log('⚙️  Setting collection...\n');

  // Note: This requires Metaplex SDK which we're avoiding for MVP
  // For now, provide manual instructions
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('📝 MANUAL VERIFICATION INSTRUCTIONS');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

  console.log('Since this is MVP, use Metaplex Sugar to verify:');
  console.log('');
  console.log('1. Install Metaplex Sugar:');
  console.log('   bash <(curl -sSf https://sugar.metaplex.com/install.sh)');
  console.log('');
  console.log('2. Verify NFT into collection:');
  console.log(`   metaplex verify_nft_collection \\`);
  console.log(`     --keypair ~/.config/solana/id.json \\`);
  console.log(`     --nft-mint ${nftMint} \\`);
  console.log(`     --collection-mint ${collectionMint} \\`);
  console.log(`     --rpc-url ${network}`);
  console.log('');
  console.log('3. Or use Solana CLI with mpl-token-metadata:');
  console.log(`   spl-token approve ${nftMint} 1 ${collectionAuthority}`);
  console.log('');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

  console.log('💡 TIP: For new mints, collection is auto-set in contract');
  console.log('This script is only for migrating old NFTs\n');
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error('❌ Error:', error);
    process.exit(1);
  });
