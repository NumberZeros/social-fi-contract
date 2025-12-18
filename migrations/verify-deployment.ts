import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { SocialFiContract } from '../target/types/social_fi_contract';

async function main() {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SocialFiContract as Program<SocialFiContract>;
  
  console.log('🔍 Verifying Deployment\n');
  console.log('Program ID:', program.programId.toBase58());
  console.log('RPC Endpoint:', provider.connection.rpcEndpoint);
  console.log('Wallet:', provider.wallet.publicKey.toBase58());
  console.log('---\n');

  // Check program account exists
  try {
    const programInfo = await provider.connection.getAccountInfo(program.programId);
    if (!programInfo) {
      console.log('❌ Program account not found!');
      return;
    }
    console.log('✅ Program account exists');
    console.log('   Executable:', programInfo.executable);
    console.log('   Owner:', programInfo.owner.toBase58());
    console.log('   Data length:', programInfo.data.length, 'bytes');
    console.log('   Lamports:', programInfo.lamports / 1e9, 'SOL');
  } catch (error) {
    console.error('❌ Error checking program account:', error);
    return;
  }

  // Check platform config
  const [platformConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from('platform_config')],
    program.programId
  );

  console.log('\n📋 Platform Config PDA:', platformConfig.toBase58());

  try {
    const account = await program.account.platformConfig.fetch(platformConfig);
    console.log('✅ Platform config initialized');
    console.log('   Admin:', account.admin.toBase58());
    console.log('   Fee Collector:', account.feeCollector.toBase58());
    console.log('   Paused:', account.paused);
    console.log('   Bump:', account.bump);
  } catch (error) {
    console.log('⚠️  Platform config not initialized');
    console.log('   Run: pnpm migration:init');
  }

  console.log('\n✅ Deployment verification complete!');
  console.log('\n📝 Program Summary:');
  console.log('   Program ID:', program.programId.toBase58());
  console.log('   Network:', provider.connection.rpcEndpoint.includes('devnet') ? 'Devnet' : 
                            provider.connection.rpcEndpoint.includes('mainnet') ? 'Mainnet' : 'Localnet');
  console.log('   Status: Ready ✨');
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
