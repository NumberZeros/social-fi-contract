import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { SocialFiContract } from "../target/types/social_fi_contract";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("social-fi-contract", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SocialFiContract as Program<SocialFiContract>;
  
  // Test wallets
  let user1: Keypair;
  let user2: Keypair;
  let creator: Keypair;
  
  // Test constants
  const USERNAME1 = "test_user_1";
  const USERNAME2 = "test_user_2";
  const CREATOR_USERNAME = "creator_pro";
  const GROUP_NAME = "TestGroup";
  const PROPOSAL_TITLE = "TestProposal";
  const NFT_USERNAME = "rare";

  before(async () => {
    // Generate test keypairs
    user1 = Keypair.generate();
    user2 = Keypair.generate();
    creator = Keypair.generate();

    // Airdrop SOL to test accounts
    const airdropAmount = 10 * LAMPORTS_PER_SOL;
    await provider.connection.requestAirdrop(user1.publicKey, airdropAmount);
    await provider.connection.requestAirdrop(user2.publicKey, airdropAmount);
    await provider.connection.requestAirdrop(creator.publicKey, airdropAmount);
    
    // Wait for airdrops to confirm
    await new Promise(resolve => setTimeout(resolve, 2000));
  });

  describe("User Profile & Tipping", () => {
    it("Initializes user profiles", async () => {
      // User 1
      const [user1Profile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), user1.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeUser(USERNAME1)
        .accounts({
          user: user1.publicKey,
          userProfile: user1Profile,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const profile = await program.account.userProfile.fetch(user1Profile);
      expect(profile.username).to.equal(USERNAME1);
      expect(profile.owner.toString()).to.equal(user1.publicKey.toString());

      // User 2
      const [user2Profile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), user2.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeUser(USERNAME2)
        .accounts({
          user: user2.publicKey,
          userProfile: user2Profile,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2])
        .rpc();

      // Creator
      const [creatorProfile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), creator.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeUser(CREATOR_USERNAME)
        .accounts({
          user: creator.publicKey,
          userProfile: creatorProfile,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();
    });

    it("Sends a tip", async () => {
      const [user1Profile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), user1.publicKey.toBuffer()],
        program.programId
      );

      const [creatorProfile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), creator.publicKey.toBuffer()],
        program.programId
      );

      const tipAmount = new BN(0.1 * LAMPORTS_PER_SOL);

      await program.methods
        .sendTip(tipAmount)
        .accounts({
          sender: user1.publicKey,
          senderProfile: user1Profile,
          recipient: creator.publicKey,
          recipientProfile: creatorProfile,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const recipientProfile = await program.account.userProfile.fetch(creatorProfile);
      expect(recipientProfile.totalTipsReceived.toNumber()).to.equal(tipAmount.toNumber());
    });
  });

  describe("Bonding Curve (Creator Shares)", () => {
    it("Initializes creator pool", async () => {
      const [creatorProfile] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), creator.publicKey.toBuffer()],
        program.programId
      );

      const [creatorPool] = PublicKey.findProgramAddressSync(
        [Buffer.from("creator_pool"), creator.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeCreatorPool()
        .accounts({
          creatorPool,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const pool = await program.account.creatorPool.fetch(creatorPool);
      expect(pool.creator.toString()).to.equal(creator.publicKey.toString());
      expect(pool.supply.toNumber()).to.equal(0);
    });

    it("Buys shares", async () => {
      const [creatorPool] = PublicKey.findProgramAddressSync(
        [Buffer.from("creator_pool"), creator.publicKey.toBuffer()],
        program.programId
      );

      // Correct order: [buyer, creator]
      const [shareHolding] = PublicKey.findProgramAddressSync(
        [Buffer.from("share_holding"), user1.publicKey.toBuffer(), creator.publicKey.toBuffer()],
        program.programId
      );

      const sharesToBuy = new BN(5);

      await program.methods
        .buyShares(sharesToBuy)
        .accounts({
          creatorPool,
          shareHolding,
          buyer: user1.publicKey,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const holding = await program.account.shareHolding.fetch(shareHolding);
      expect(holding.sharesOwned.toNumber()).to.equal(sharesToBuy.toNumber());

      const pool = await program.account.creatorPool.fetch(creatorPool);
      expect(pool.supply.toNumber()).to.equal(sharesToBuy.toNumber());
    });

    it("Sells shares", async () => {
      const [creatorPool] = PublicKey.findProgramAddressSync(
        [Buffer.from("creator_pool"), creator.publicKey.toBuffer()],
        program.programId
      );

      const [shareHolding] = PublicKey.findProgramAddressSync(
        [Buffer.from("share_holding"), user1.publicKey.toBuffer(), creator.publicKey.toBuffer()],
        program.programId
      );

      const sharesToSell = new BN(2);

      await program.methods
        .sellShares(sharesToSell)
        .accounts({
          creatorPool,
          shareHolding,
          seller: user1.publicKey,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const holding = await program.account.shareHolding.fetch(shareHolding);
      expect(holding.sharesOwned.toNumber()).to.equal(3);
    });
  });

  describe("Subscription System", () => {
    it("Creates subscription tier", async () => {
      const name = "Premium";
      const description = "Exclusive";
      const price = 500_000_000; // 0.5 SOL in lamports
      const durationDays = 30;

      // Use tier_id = 1 for PDA derivation (same as get_next_tier_id())
      const tierId = new BN(1);
      const [subscriptionTier] = PublicKey.findProgramAddressSync(
        [Buffer.from("subscription_tier"), creator.publicKey.toBuffer(), tierId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      await program.methods
        .createSubscriptionTier(name, description, new BN(price), new BN(durationDays))
        .accounts({
          subscriptionTier,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const tier = await program.account.subscriptionTier.fetch(subscriptionTier);
      expect(tier.price.toNumber()).to.equal(price);
      expect(tier.name).to.equal(name);
    });

    it("Subscribes to tier", async () => {
      const tierId = new BN(1);
      const [subscriptionTier] = PublicKey.findProgramAddressSync(
        [Buffer.from("subscription_tier"), creator.publicKey.toBuffer(), tierId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      const [subscription] = PublicKey.findProgramAddressSync(
        [Buffer.from("subscription"), user1.publicKey.toBuffer(), subscriptionTier.toBuffer()],
        program.programId
      );

      await program.methods
        .subscribe()
        .accounts({
          subscription,
          subscriptionTier,
          subscriber: user1.publicKey,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const sub = await program.account.subscription.fetch(subscription);
      expect(sub.subscriber.toString()).to.equal(user1.publicKey.toString());
    });

    it("Cancels subscription", async () => {
      const tierId = new BN(1);
      const [subscriptionTier] = PublicKey.findProgramAddressSync(
        [Buffer.from("subscription_tier"), creator.publicKey.toBuffer(), tierId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      const [subscription] = PublicKey.findProgramAddressSync(
        [Buffer.from("subscription"), user1.publicKey.toBuffer(), subscriptionTier.toBuffer()],
        program.programId
      );

      await program.methods
        .cancelSubscription()
        .accounts({
          subscription,
          subscriber: user1.publicKey,
        })
        .signers([user1])
        .rpc();

      const sub = await program.account.subscription.fetch(subscription);
      expect(sub.cancelled).to.be.true;
    });
  });

  describe("Group Management", () => {
    it("Creates a group", async () => {
      const [group] = PublicKey.findProgramAddressSync(
        [Buffer.from("group"), creator.publicKey.toBuffer(), Buffer.from(GROUP_NAME)],
        program.programId
      );

      const [groupMember] = PublicKey.findProgramAddressSync(
        [Buffer.from("group_member"), group.toBuffer(), creator.publicKey.toBuffer()],
        program.programId
      );

      // privacy: 0 = public, entry_requirement: 0 = free, entry_price: none
      await program.methods
        .createGroup(GROUP_NAME, "Test description", 0, 0, null)
        .accounts({
          group,
          groupMember,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const groupAccount = await program.account.group.fetch(group);
      expect(groupAccount.name).to.equal(GROUP_NAME);
    });

    it("Joins group", async () => {
      const [group] = PublicKey.findProgramAddressSync(
        [Buffer.from("group"), creator.publicKey.toBuffer(), Buffer.from(GROUP_NAME)],
        program.programId
      );

      const [groupMember] = PublicKey.findProgramAddressSync(
        [Buffer.from("group_member"), group.toBuffer(), user1.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .joinGroup()
        .accounts({
          group,
          groupMember,
          groupCreator: creator.publicKey,
          member: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const member = await program.account.groupMember.fetch(groupMember);
      expect(member.wallet.toString()).to.equal(user1.publicKey.toString());
    });

    it("Updates member role", async () => {
      const [group] = PublicKey.findProgramAddressSync(
        [Buffer.from("group"), creator.publicKey.toBuffer(), Buffer.from(GROUP_NAME)],
        program.programId
      );

      const [targetMember] = PublicKey.findProgramAddressSync(
        [Buffer.from("group_member"), group.toBuffer(), user1.publicKey.toBuffer()],
        program.programId
      );

      const [authorityMember] = PublicKey.findProgramAddressSync(
        [Buffer.from("group_member"), group.toBuffer(), creator.publicKey.toBuffer()],
        program.programId
      );

      // 2 = moderator
      await program.methods
        .updateMemberRole(2)
        .accounts({
          group,
          targetMember,
          authorityMember,
          authority: creator.publicKey,
        })
        .signers([creator])
        .rpc();

      const member = await program.account.groupMember.fetch(targetMember);
      expect(member.role).to.equal(2);
    });
  });

  describe("Governance (Staking & Voting)", () => {
    it("Stakes tokens", async () => {
      const [stakePosition] = PublicKey.findProgramAddressSync(
        [Buffer.from("stake_position"), user1.publicKey.toBuffer()],
        program.programId
      );

      const stakeAmount = new BN(100 * LAMPORTS_PER_SOL);
      const lockPeriod = new BN(90); // 90 days lock period

      await program.methods
        .stakeTokens(stakeAmount, lockPeriod)
        .accounts({
          stakePosition,
          staker: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const stake = await program.account.stakePosition.fetch(stakePosition);
      expect(stake.amount.toString()).to.equal(stakeAmount.toString());
      expect(stake.lockPeriod.toNumber()).to.equal(90);
    });

    it("Creates proposal", async () => {
      const [stakePosition] = PublicKey.findProgramAddressSync(
        [Buffer.from("stake_position"), user1.publicKey.toBuffer()],
        program.programId
      );

      const [proposal] = PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), user1.publicKey.toBuffer(), Buffer.from(PROPOSAL_TITLE)],
        program.programId
      );

      const category = 0; // u8 category
      const executionDelay = new BN(86400); // 1 day in seconds (i64)

      await program.methods
        .createProposal(PROPOSAL_TITLE, "Test description", category, executionDelay)
        .accounts({
          proposal,
          stakePosition,
          proposer: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const proposalAccount = await program.account.proposal.fetch(proposal);
      expect(proposalAccount.title).to.equal(PROPOSAL_TITLE);
    });

    it("Casts vote", async () => {
      const [proposal] = PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), user1.publicKey.toBuffer(), Buffer.from(PROPOSAL_TITLE)],
        program.programId
      );

      const [stakePosition] = PublicKey.findProgramAddressSync(
        [Buffer.from("stake_position"), user1.publicKey.toBuffer()],
        program.programId
      );

      const [vote] = PublicKey.findProgramAddressSync(
        [Buffer.from("vote"), proposal.toBuffer(), user1.publicKey.toBuffer()],
        program.programId
      );

      // 0 = For
      await program.methods
        .castVote(0)
        .accounts({
          vote,
          proposal,
          stakePosition,
          voter: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const voteAccount = await program.account.vote.fetch(vote);
      expect(voteAccount.voter.toString()).to.equal(user1.publicKey.toString());
    });
  });

  describe("Username NFT Marketplace", () => {
    it("Mints username NFT", async () => {
      const [usernameNft] = PublicKey.findProgramAddressSync(
        [Buffer.from("username_nft"), Buffer.from(NFT_USERNAME)],
        program.programId
      );

      await program.methods
        .mintUsername(NFT_USERNAME)
        .accounts({
          usernameNft,
          minter: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const nft = await program.account.usernameNft.fetch(usernameNft);
      expect(nft.username).to.equal(NFT_USERNAME);
      expect(nft.owner.toString()).to.equal(user1.publicKey.toString());
    });

    it("Lists username NFT", async () => {
      const [usernameNft] = PublicKey.findProgramAddressSync(
        [Buffer.from("username_nft"), Buffer.from(NFT_USERNAME)],
        program.programId
      );

      const [listing] = PublicKey.findProgramAddressSync(
        [Buffer.from("listing"), usernameNft.toBuffer()],
        program.programId
      );

      const price = new BN(1 * LAMPORTS_PER_SOL);

      await program.methods
        .listUsername(price)
        .accounts({
          usernameNft,
          listing,
          seller: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const listingAccount = await program.account.listing.fetch(listing);
      expect(listingAccount.price.toString()).to.equal(price.toString());
    });

    it("Makes offer", async () => {
      const [usernameNft] = PublicKey.findProgramAddressSync(
        [Buffer.from("username_nft"), Buffer.from(NFT_USERNAME)],
        program.programId
      );

      const [listing] = PublicKey.findProgramAddressSync(
        [Buffer.from("listing"), usernameNft.toBuffer()],
        program.programId
      );

      const [offer] = PublicKey.findProgramAddressSync(
        [Buffer.from("offer"), listing.toBuffer(), user2.publicKey.toBuffer()],
        program.programId
      );

      const offerPrice = new BN(0.8 * LAMPORTS_PER_SOL);

      await program.methods
        .makeOffer(offerPrice)
        .accounts({
          offer,
          listing,
          buyer: user2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2])
        .rpc();

      const offerAccount = await program.account.offer.fetch(offer);
      expect(offerAccount.price.toString()).to.equal(offerPrice.toString());
    });

    it("Accepts offer", async () => {
      const [usernameNft] = PublicKey.findProgramAddressSync(
        [Buffer.from("username_nft"), Buffer.from(NFT_USERNAME)],
        program.programId
      );

      const [listing] = PublicKey.findProgramAddressSync(
        [Buffer.from("listing"), usernameNft.toBuffer()],
        program.programId
      );

      const [offer] = PublicKey.findProgramAddressSync(
        [Buffer.from("offer"), listing.toBuffer(), user2.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .acceptOffer()
        .accounts({
          usernameNft,
          listing,
          offer,
          seller: user1.publicKey,
          buyer: user2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      // Verify NFT ownership transferred
      const nft = await program.account.usernameNft.fetch(usernameNft);
      expect(nft.owner.toString()).to.equal(user2.publicKey.toString());
    });
  });
});
