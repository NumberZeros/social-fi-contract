import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { SocialFiContract } from '../target/types/social_fi_contract';

async function main() {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SocialFiContract as Program<SocialFiContract>;
  
  console.log('Program ID:', program.programId.toBase58());
  console.log('Admin wallet:', provider.wallet.publicKey.toBase58());

  // Derive platform config PDA
  const [platformConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from('platform_config')],
    program.programId
  );

  console.log('Platform config PDA:', platformConfig.toBase58());

  // Check if already initialized
  try {
    const account = await program.account.platformConfig.fetch(platformConfig);
    console.log('✅ Platform already initialized');
    console.log('Admin:', account.admin.toBase58());
    console.log('Fee collector:', account.feeCollector.toBase58());
    console.log('Paused:', account.paused);
    return;
  } catch (error) {
    console.log('Platform not initialized, initializing...');
  }

  // Initialize platform with admin as fee collector
  try {
    const tx = await program.methods
      .initializePlatform(provider.wallet.publicKey)
      .accountsPartial({
        platformConfig,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log('✅ Platform initialized!');
    console.log('Transaction signature:', tx);
    
    // Verify
    const account = await program.account.platformConfig.fetch(platformConfig);
    console.log('\nPlatform Config:');
    console.log('Admin:', account.admin.toBase58());
    console.log('Fee collector:', account.feeCollector.toBase58());
    console.log('Paused:', account.paused);
  } catch (error) {
    console.error('❌ Error initializing platform:', error);
    throw error;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
