# Social-Fi Smart Contract - API Reference

## Program Overview

The Social-Fi smart contract provides blockchain infrastructure for a decentralized social platform with creator monetization, governance, and marketplace features.

**Program ID:** `FHHfGX8mYxagDmhsXgJUfLnx1rw2M138e3beCwWELdgL`

## Table of Contents

1. [User & Tipping](#user--tipping)
2. [Creator Shares](#creator-shares)
3. [Subscriptions](#subscriptions)
4. [Groups](#groups)
5. [Governance](#governance)
6. [Marketplace](#marketplace)
7. [Account Structures](#account-structures)
8. [Events](#events)
9. [Errors](#errors)

---

## User & Tipping

### `initialize_user`

Initialize a user profile with a unique username.

**Parameters:**
- `username: String` - Unique username (max 20 chars, alphanumeric + underscore)

**Accounts:**
- `user_profile` - PDA (init) `[USER_PROFILE_SEED, user.key()]`
- `user` - Signer, payer
- `system_program`

**Validation:**
- Username max 20 characters
- Alphanumeric + underscore only
- Referral code generated from wallet address

**Emits:** `UserInitialized`

**Example:**
```typescript
await program.methods
  .initializeUser("alice_2025")
  .accounts({
    userProfile,
    user: wallet.publicKey,
    systemProgram,
  })
  .rpc();
```

---

### `send_tip`

Send SOL tip to another user.

**Parameters:**
- `amount: u64` - Tip amount in lamports

**Accounts:**
- `sender_profile` - PDA `[USER_PROFILE_SEED, sender.key()]`
- `recipient_profile` - PDA `[USER_PROFILE_SEED, recipient.key()]`
- `sender` - Signer
- `recipient` - Tip recipient wallet
- `system_program`

**Validation:**
- Amount > 0
- Cannot tip yourself
- Both users must have initialized profiles

**Emits:** `TipSent`

**Example:**
```typescript
await program.methods
  .sendTip(new BN(1_000_000_000)) // 1 SOL
  .accounts({
    senderProfile,
    recipientProfile,
    sender: wallet.publicKey,
    recipient: recipientWallet,
    systemProgram,
  })
  .rpc();
```

---

## Creator Shares

### `initialize_creator_pool`

Initialize bonding curve for creator shares.

**Accounts:**
- `creator_pool` - PDA (init) `[CREATOR_POOL_SEED, creator.key()]`
- `creator` - Signer, payer
- `system_program`

**Initial State:**
- Supply: 0
- Base price: 0.01 SOL
- Holders: 0

**Emits:** None

---

### `buy_shares`

Purchase creator shares using bonding curve pricing.

**Parameters:**
- `amount: u64` - Number of shares to buy

**Accounts:**
- `creator_pool` - PDA `[CREATOR_POOL_SEED, creator.key()]`
- `share_holding` - PDA (init_if_needed) `[SHARE_HOLDING_SEED, buyer.key(), creator.key()]`
- `buyer` - Signer, payer
- `creator` - Creator wallet (receives payment)
- `system_program`

**Pricing Formula:**
```
price(supply) = basePrice × (supply / 100)²
total_cost = Σ price(supply + i) for i in 0..amount
```

**Validation:**
- Amount > 0
- Buyer has sufficient balance

**Emits:** `SharesPurchased`

---

### `sell_shares`

Sell creator shares (10% fee).

**Parameters:**
- `amount: u64` - Number of shares to sell

**Accounts:**
- `creator_pool` - PDA `[CREATOR_POOL_SEED, creator.key()]`
- `share_holding` - PDA `[SHARE_HOLDING_SEED, seller.key(), creator.key()]`
- `seller` - Signer
- `creator` - Creator wallet (provides refund)
- `system_program`

**Fee Structure:**
- 10% of sell value goes to creator
- Seller receives 90%

**Validation:**
- Amount > 0
- Seller holds sufficient shares

**Emits:** `SharesSold`

---

## Subscriptions

### `create_subscription_tier`

Create a subscription tier as a creator.

**Parameters:**
- `name: String` - Tier name (max 50 chars)
- `description: String` - Description (max 500 chars)
- `price: u64` - Monthly price in lamports
- `duration_days: u64` - Subscription duration

**Accounts:**
- `subscription_tier` - PDA (init) `[SUBSCRIPTION_TIER_SEED, creator.key(), tier_id]`
- `creator` - Signer, payer
- `system_program`

**Validation:**
- Price > 0
- Duration > 0
- Name and description length limits

**Emits:** `SubscriptionTierCreated`

---

### `subscribe`

Subscribe to a creator's tier.

**Accounts:**
- `subscription_tier` - PDA `[SUBSCRIPTION_TIER_SEED, creator.key(), tier_id]`
- `subscription` - PDA (init) `[SUBSCRIPTION_SEED, subscriber.key(), creator.key(), tier_id]`
- `subscriber` - Signer, payer
- `creator` - Creator wallet (receives payment)
- `system_program`

**Payment:**
- Transfers tier price to creator
- Sets expiry date based on duration

**Emits:** `UserSubscribed`

---

### `cancel_subscription`

Cancel an active subscription.

**Accounts:**
- `subscription` - PDA `[SUBSCRIPTION_SEED, subscriber.key(), creator.key(), tier_id]`
- `subscriber` - Signer

**Validation:**
- Subscription must be active
- Only subscriber can cancel

**Emits:** `SubscriptionCancelled`

---

## Groups

### `create_group`

Create a new group.

**Parameters:**
- `name: String` - Group name (max 50 chars)
- `description: String` - Description (max 500 chars)
- `privacy: u8` - 0=public, 1=private, 2=secret
- `entry_requirement: u8` - 0=free, 1=pay_sol, 2=hold_token, 3=hold_nft
- `entry_price: Option<u64>` - Required if pay_sol

**Accounts:**
- `group` - PDA (init) `[GROUP_SEED, creator.key(), name.as_bytes()]`
- `group_member` - PDA (init) `[GROUP_MEMBER_SEED, group.key(), creator.key()]`
- `creator` - Signer, payer (becomes owner)
- `system_program`

**Emits:** `GroupCreated`, `MemberJoined`

---

### `join_group`

Join a group.

**Accounts:**
- `group` - PDA `[GROUP_SEED, creator.key(), group.name]`
- `group_member` - PDA (init) `[GROUP_MEMBER_SEED, group.key(), member.key()]`
- `member` - Signer, payer
- `group_creator` - Group creator wallet (receives entry fee)
- `system_program`

**Entry Requirements:**
- Free: No payment
- Pay SOL: Transfer entry_price
- Hold Token/NFT: Verification (placeholder)

**Emits:** `MemberJoined`

---

### `update_member_role`

Update a member's role (admin/mod only).

**Parameters:**
- `new_role: u8` - 1=admin, 2=moderator, 3=member

**Accounts:**
- `group` - PDA
- `admin_member` - PDA (must be owner/admin)
- `target_member` - PDA (member to update)
- `admin` - Signer

**Validation:**
- Admin has manage_members permission
- Cannot update own role

**Emits:** `MemberRoleUpdated`

---

### `kick_member`

Remove a member from the group.

**Accounts:**
- `group` - PDA
- `admin_member` - PDA (must be owner/admin)
- `target_member` - PDA (closes account)
- `admin` - Signer

**Validation:**
- Admin has manage_members permission
- Cannot kick yourself

**Emits:** `MemberKicked`

---

## Governance

### `stake_tokens`

Stake tokens with lock period for voting power.

**Parameters:**
- `amount: u64` - Tokens to stake
- `lock_period: u64` - Days (0, 30, 90, 180, or 365)

**Lock Multipliers:**
| Days | APY  | Voting Power |
|------|------|--------------|
| 0    | 5%   | 1.0x         |
| 30   | 10%  | 1.2x         |
| 90   | 15%  | 1.5x         |
| 180  | 20%  | 2.0x         |
| 365  | 30%  | 3.0x         |

**Accounts:**
- `stake_position` - PDA (init) `[STAKE_POSITION_SEED, staker.key()]`
- `staker` - Signer, payer
- `system_program`

**Emits:** `TokensStaked`

---

### `unstake_tokens`

Unstake tokens after lock period.

**Accounts:**
- `stake_position` - PDA (closes)
- `staker` - Signer

**Validation:**
- Lock period must be complete
- Rewards calculated based on APY

**Emits:** `TokensUnstaked`

---

### `create_proposal`

Create a governance proposal.

**Parameters:**
- `title: String` - Proposal title (max 100 chars)
- `description: String` - Details (max 500 chars)
- `category: u8` - 0=protocol, 1=treasury, 2=feature, 3=parameter
- `execution_delay: i64` - Timelock seconds (min 24 hours)

**Accounts:**
- `proposal` - PDA (init) `[PROPOSAL_SEED, proposer.key(), title.as_bytes()]`
- `stake_position` - PDA (must have min voting power)
- `proposer` - Signer, payer
- `system_program`

**Requirements:**
- Minimum 1000 voting power
- Voting period: 7 days
- Quorum: 10% of total staked

**Emits:** `ProposalCreated`

---

### `cast_vote`

Vote on a proposal.

**Parameters:**
- `vote_type: u8` - 0=for, 1=against, 2=abstain

**Accounts:**
- `proposal` - PDA
- `vote` - PDA (init) `[VOTE_SEED, proposal.key(), voter.key()]`
- `stake_position` - PDA (voting power source)
- `voter` - Signer, payer
- `system_program`

**Validation:**
- Voting period active
- Cannot vote twice
- Voting power from staked tokens

**Emits:** `VoteCast`

---

### `execute_proposal`

Execute a passed proposal.

**Accounts:**
- `proposal` - PDA
- `executor` - Signer

**Validation:**
- Voting period ended
- Proposal passed (for > against)
- Quorum met
- Execution delay elapsed

**Emits:** `ProposalExecuted`

---

## Marketplace

### `mint_username`

Mint a username NFT.

**Parameters:**
- `username: String` - Unique username (max 20 chars)

**Accounts:**
- `username_nft` - PDA (init) `[USERNAME_NFT_SEED, username.as_bytes()]`
- `owner` - Signer, payer
- `system_program`

**Validation:**
- Alphanumeric + underscore
- Globally unique

**Emits:** `UsernameMinted`

---

### `list_username`

List username NFT for sale.

**Parameters:**
- `price: u64` - Sale price in lamports

**Accounts:**
- `username_nft` - PDA (must own)
- `listing` - PDA (init) `[LISTING_SEED, username_nft.key()]`
- `seller` - Signer, payer
- `system_program`

**Auto-categorization:**
- ≤3 chars: Rare
- ≤5 chars: Short
- \>5 chars: Custom

**Emits:** `UsernameListed`

---

### `buy_listing`

Purchase a listed username.

**Accounts:**
- `username_nft` - PDA (transfers ownership)
- `listing` - PDA (closes)
- `buyer` - Signer, payer
- `seller` - Listing seller (receives payment)
- `system_program`

**Process:**
1. Transfer payment to seller
2. Transfer NFT ownership to buyer
3. Close listing account

**Emits:** `UsernameSold`

---

### `make_offer`

Make an offer on a listing.

**Parameters:**
- `amount: u64` - Offer amount in lamports

**Accounts:**
- `listing` - PDA
- `offer` - PDA (init) `[OFFER_SEED, listing.key(), buyer.key()]`
- `buyer` - Signer, payer
- `system_program`

**Expiry:** 7 days

**Emits:** `OfferMade`

---

### `accept_offer`

Accept an offer (seller only).

**Accounts:**
- `username_nft` - PDA (transfers ownership)
- `listing` - PDA (closes)
- `offer` - PDA (closes)
- `seller` - Signer (listing owner)
- `buyer` - Offer maker (receives NFT)
- `system_program`

**Validation:**
- Offer not expired
- Seller owns listing

**Emits:** `OfferAccepted`

---

## Account Structures

### UserProfile
```rust
{
  owner: Pubkey,
  username: String,
  total_tips_sent: u64,
  total_tips_received: u64,
  posts_count: u64,
  followers_count: u64,
  following_count: u64,
  referral_code: String,
  referred_by: Option<Pubkey>,
  referrals_count: u64,
  created_at: i64,
  bump: u8,
}
```

### CreatorPool
```rust
{
  creator: Pubkey,
  supply: u64,
  holders_count: u64,
  base_price: u64,
  total_volume: u64,
  created_at: i64,
  bump: u8,
}
```

### SubscriptionTier
```rust
{
  creator: Pubkey,
  tier_id: u64,
  name: String,
  description: String,
  price: u64,
  duration_days: u64,
  subscriber_count: u64,
  created_at: i64,
  bump: u8,
}
```

### Group
```rust
{
  id: Pubkey,
  name: String,
  description: String,
  creator: Pubkey,
  privacy: u8,
  entry_requirement: u8,
  entry_price: Option<u64>,
  token_mint: Option<Pubkey>,
  nft_collection: Option<Pubkey>,
  member_count: u64,
  post_count: u64,
  created_at: i64,
  bump: u8,
}
```

### StakePosition
```rust
{
  staker: Pubkey,
  amount: u64,
  staked_at: i64,
  lock_period: u64,
  unlocks_at: i64,
  rewards: u64,
  voting_power: u64,
  bump: u8,
}
```

### Proposal
```rust
{
  id: Pubkey,
  proposer: Pubkey,
  title: String,
  description: String,
  category: u8,
  status: u8,
  created_at: i64,
  voting_ends_at: i64,
  execution_delay: i64,
  votes_for: u64,
  votes_against: u64,
  votes_abstain: u64,
  quorum_required: u64,
  executed_at: Option<i64>,
  bump: u8,
}
```

### UsernameNFT
```rust
{
  owner: Pubkey,
  username: String,
  verified: bool,
  minted_at: i64,
  bump: u8,
}
```

---

## Events

23 events emitted for comprehensive tracking:

- `UserInitialized`
- `TipSent`
- `SharesPurchased`
- `SharesSold`
- `SubscriptionTierCreated`
- `UserSubscribed`
- `SubscriptionCancelled`
- `GroupCreated`
- `MemberJoined`
- `MemberRoleUpdated`
- `MemberKicked`
- `TokensStaked`
- `TokensUnstaked`
- `ProposalCreated`
- `VoteCast`
- `ProposalExecuted`
- `UsernameMinted`
- `UsernameListed`
- `UsernameSold`
- `OfferMade`
- `OfferAccepted`

---

## Errors

40+ custom error codes including:

- `UsernameTooLong`
- `UsernameAlreadyTaken`
- `InvalidAmount`
- `CannotTipSelf`
- `BondingCurveOverflow`
- `InsufficientShares`
- `InsufficientPermissions`
- `TokensLocked`
- `InsufficientVotingPower`
- `QuorumNotReached`
- `VotingPeriodEnded`
- And more...

---

## Gas Estimates

| Instruction | Compute Units | Typical Cost |
|-------------|---------------|--------------|
| `initialize_user` | ~10k | ~0.000005 SOL |
| `send_tip` | ~5k | ~0.000002 SOL |
| `buy_shares` | ~15k | ~0.000008 SOL |
| `subscribe` | ~8k | ~0.000004 SOL |
| `create_group` | ~12k | ~0.000006 SOL |
| `stake_tokens` | ~10k | ~0.000005 SOL |
| `cast_vote` | ~8k | ~0.000004 SOL |

*Note: Costs exclude account rent and transfers*

---

## Security Considerations

1. **Reentrancy:** Not applicable (Solana's account model)
2. **Integer Overflow:** All arithmetic uses checked operations
3. **Access Control:** Role-based permissions enforced
4. **Input Validation:** Comprehensive length and format checks
5. **PDA Security:** All seeds properly validated
6. **Rent Exemption:** All accounts sized for rent exemption

---

## Testing

Run integration tests:
```bash
anchor test
```

Run specific test:
```bash
anchor test --skip-deploy -- --test initialize_user
```

---

## Deployment

Deploy to devnet:
```bash
anchor deploy --provider.cluster devnet
```

Verify deployment:
```bash
solana program show FHHfGX8mYxagDmhsXgJUfLnx1rw2M138e3beCwWELdgL
```
