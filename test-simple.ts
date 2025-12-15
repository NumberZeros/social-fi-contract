import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SocialFiContract } from "./target/types/social_fi_contract";
import { PublicKey, SystemProgram } from "@solana/web3.js";

const provider = anchor.AnchorProvider.env();
const program = anchor.workspace.SocialFiContract as Program<SocialFiContract>;

const [platformConfig] = PublicKey.findProgramAddressSync(
  [Buffer.from("platform_config")],
  program.programId
);

// Thử call với format đúng
await program.methods
  .initializePlatform(provider.wallet.publicKey)
  .accounts({
    platformConfig,
    admin: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
