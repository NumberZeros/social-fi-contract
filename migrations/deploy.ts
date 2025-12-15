import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { SocialFiContract } from "../target/types/social_fi_contract";

/**
 * ADMIN CONFIGURATION
 * 
 * Change this to your desired admin wallet address.
 * The admin has full control over platform operations:
 * - Pause/unpause platform
 * - Update fee collector
 * - Update liquidity requirements
 * - Transfer admin rights
 * 
 * For production, use a multisig wallet (e.g., Squads 3-of-5)
 */
const ADMIN_WALLET = new PublicKey("6s3NXs6inignnQiyiZCED3MCAb6U4pUooTACgiAa2E2k");

/**
 * FEE COLLECTOR CONFIGURATION
 * 
 * Address that receives platform fees from:
 * - Share trading fees
 * - Marketplace listing fees
 * - Username minting fees
 * 
 * Can be changed later via update_fee_collector instruction
 */
const FEE_COLLECTOR = ADMIN_WALLET; // Same as admin by default, can be different

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);

  console.log("🚀 Starting Social-Fi Platform Deployment");
  console.log("=" .repeat(60));
  console.log("📍 Network:", provider.connection.rpcEndpoint);
  console.log("👤 Deployer:", provider.wallet.publicKey.toString());
  console.log("🔑 Admin:", ADMIN_WALLET.toString());
  console.log("💰 Fee Collector:", FEE_COLLECTOR.toString());
  console.log("=" .repeat(60));

  const program = anchor.workspace.SocialFiContract as Program<SocialFiContract>;

  // ==================== Initialize Platform Config ====================
  console.log("\n📦 Step 1: Initializing Platform Configuration...");
  
  const [platformConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("platform_config")],
    program.programId
  );

  console.log("   Platform Config PDA:", platformConfig.toString());

  try {
    // Check if already initialized
    const existingConfig = await program.account.platformConfig.fetch(platformConfig);
    console.log("\n✅ Platform config already initialized!");
    console.log("   Admin:", existingConfig.admin.toString());
    console.log("   Fee Collector:", existingConfig.feeCollector.toString());
    console.log("   Paused:", existingConfig.paused);
    console.log("   Min Liquidity BPS:", existingConfig.minLiquidityBps.toString(), "(10%)");
    
    if (!existingConfig.admin.equals(ADMIN_WALLET)) {
      console.log("\n⚠️  WARNING: Admin mismatch!");
      console.log("   Expected:", ADMIN_WALLET.toString());
      console.log("   Actual:", existingConfig.admin.toString());
      console.log("   Run 'update_admin' instruction to change admin");
    }
  } catch (e) {
    // Not initialized yet - proceed with initialization
    console.log("   Initializing for the first time...");
    
    // Check deployer balance
    const balance = await provider.connection.getBalance(provider.wallet.publicKey);
    console.log("   Deployer balance:", (balance / 1e9).toFixed(4), "SOL");
    
    if (balance < 0.05 * 1e9) {
      console.log("\n❌ ERROR: Insufficient balance!");
      console.log("   Need at least 0.05 SOL for initialization");
      if (provider.connection.rpcEndpoint.includes("devnet")) {
        console.log("   Request airdrop:");
        console.log(`   solana airdrop 2 ${provider.wallet.publicKey.toString()} --url devnet`);
      }
      throw new Error("Insufficient balance");
    }

    // Note: Deployer pays for initialization, but admin will control platform
    const tx = await program.methods
      .initializePlatform(FEE_COLLECTOR)
      .accountsPartial({
        platformConfig,
        admin: ADMIN_WALLET,  // Admin is set to configured address
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("\n✅ Platform config initialized successfully!");
    console.log("   Transaction:", tx);
    console.log("   PDA:", platformConfig.toString());
    console.log("   Admin:", ADMIN_WALLET.toString());
    console.log("   Fee Collector:", FEE_COLLECTOR.toString());
  }

  // ==================== Post-Deployment Checklist ====================
  console.log("\n" + "=".repeat(60));
  console.log("📋 POST-DEPLOYMENT CHECKLIST");
  console.log("=".repeat(60));
  console.log("✅ Platform config initialized");
  console.log("✅ Admin wallet:", ADMIN_WALLET.toString());
  console.log("✅ Fee collector:", FEE_COLLECTOR.toString());
  console.log("\n⚠️  SECURITY REMINDERS:");
  console.log("   - Admin wallet controls pause/unpause");
  console.log("   - Admin can change fee collector");
  console.log("   - Admin can transfer admin rights");
  console.log("   - For mainnet: USE MULTISIG (Squads recommended)");
  console.log("\n🎉 Deployment complete!");
  console.log("=".repeat(60));
};
