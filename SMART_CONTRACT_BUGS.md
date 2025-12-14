# Smart Contract Bugs Found During Testing

## Overview
During comprehensive integration testing, we discovered **3 bugs in the Rust smart contract** that prevent tests from passing. These require fixes in the Solana program code, not the tests.

---

## Bug #1: Sell Shares - Missing Liquidity Vault ❌

**Location:** `programs/social-fi-contract/src/instructions/shares.rs` - `sell_shares()` function

**Severity:** HIGH - Blocks core functionality

**Description:**
The `sell_shares` function attempts to transfer SOL from the creator's personal account to the seller, but the creator is not a signer of the transaction. This causes a "Cross-program invocation with unauthorized signer" error.

**Current Code:**
```rust
// Transfer from creator to seller (creator received funds on buy)
let cpi_context = CpiContext::new(
    ctx.accounts.system_program.to_account_info(),
    Transfer {
        from: ctx.accounts.creator.to_account_info(), // ❌ creator is not a signer!
        to: ctx.accounts.seller.to_account_info(),
    },
);
transfer(cpi_context, seller_receives)?;
```

**Problem:**
- When users buy shares, funds go TO the creator's wallet
- When users sell shares, the contract tries to pull funds FROM the creator's wallet
- The creator didn't sign the sell transaction, so this fails
- This is a fundamental architecture flaw

**Required Fix:**
Implement a **program-owned liquidity vault** (PDA) to hold the bonding curve funds:

```rust
// Add to shares.rs
#[derive(Accounts)]
pub struct InitializeCreatorPool<'info> {
    // ... existing accounts ...
    
    /// Program-owned vault to hold liquidity
    #[account(
        init,
        payer = creator,
        space = 8,
        seeds = [POOL_VAULT_SEED, creator.key().as_ref()],
        bump
    )]
    pub pool_vault: SystemAccount<'info>,
}

// Update BuyShares to send funds to vault instead of creator
Transfer {
    from: buyer.to_account_info(),
    to: pool_vault.to_account_info(), // ✅ Send to vault
}

// Update SellShares to pull from vault
Transfer {
    from: pool_vault.to_account_info(), // ✅ Pull from vault
    to: seller.to_account_info(),
}
```

**Alternative Quick Fix:**
If a vault is too complex, change the sell mechanism to issue IOUs or require creator approval.

**Test Impact:**
- ❌ `Sells shares` test fails with authorization error
- This blocks the bonding curve sell functionality entirely

**Error Message:**
```
Transaction simulation failed: Error processing Instruction 0: 
Cross-program invocation with unauthorized signer or writable account
Program log: 7WagyLvhzCgVFTYT53xpU1owsEpSPommbriLRuyAxryM's signer privilege escalated
```

---

## Bug #2: Make Offer - Incorrect Listing PDA Seeds ❌

**Location:** `programs/social-fi-contract/src/instructions/marketplace.rs` - `MakeOffer` struct

**Severity:** HIGH - Breaks marketplace offers

**Description:**
The `MakeOffer` instruction uses incorrect PDA seeds for the listing account, causing a seed constraint violation.

**Current Code:**
```rust
#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(
        seeds = [LISTING_SEED, listing.seller.as_ref()], // ❌ WRONG SEEDS!
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    // ...
}
```

**Correct Seeds (used in ListUsername and BuyListing):**
```rust
#[account(
    seeds = [LISTING_SEED, username_nft.key().as_ref()], // ✅ CORRECT
    bump = listing.bump
)]
pub listing: Account<'info, Listing>,
```

**Problem:**
- `ListUsername` creates listing with seeds: `[LISTING_SEED, username_nft.key()]`
- `BuyListing` reads listing with seeds: `[LISTING_SEED, username_nft.key()]` ✅
- `MakeOffer` reads listing with seeds: `[LISTING_SEED, listing.seller]` ❌ MISMATCH!
- This causes "ConstraintSeeds violated" error

**Required Fix:**
Update `MakeOffer` struct to match the PDA derivation:

```rust
#[derive(Accounts)]
pub struct MakeOffer<'info> {
    // Need to add username_nft account
    #[account(
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(
        seeds = [LISTING_SEED, username_nft.key().as_ref()], // ✅ Fixed!
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    
    // ... rest remains the same
}
```

**Test Impact:**
- ❌ `Makes offer` test fails with seed constraint error
- ❌ `Accepts offer` test fails as cascade (no offer was created)

**Error Message:**
```
Error: AnchorError caused by account: listing. 
Error Code: ConstraintSeeds. Error Number: 2006. 
Error Message: A seeds constraint was violated.
```

---

## Bug #3: Accept Offer - Same Listing PDA Issue ❌

**Location:** `programs/social-fi-contract/src/instructions/marketplace.rs` - `AcceptOffer` struct

**Severity:** HIGH - Breaks marketplace offers

**Description:**
Same issue as Bug #2 - `AcceptOffer` also uses incorrect listing PDA seeds.

**Current Code:**
```rust
#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    // ... username_nft account ...
    
    #[account(
        mut,
        seeds = [LISTING_SEED, listing.seller.as_ref()], // ❌ WRONG!
        bump = listing.bump,
        close = seller
    )]
    pub listing: Account<'info, Listing>,
    // ...
}
```

**Required Fix:**
```rust
#[account(
    mut,
    seeds = [LISTING_SEED, username_nft.key().as_ref()], // ✅ Fixed!
    bump = listing.bump,
    close = seller
)]
pub listing: Account<'info, Listing>,
```

**Note:** `username_nft` account is already present in AcceptOffer, so this is an easier fix than MakeOffer.

---

## Summary

| Bug | Location | Severity | Fix Complexity | Tests Affected |
|-----|----------|----------|----------------|----------------|
| Sell Shares Vault | shares.rs | HIGH | Medium | 1 test |
| Make Offer PDA | marketplace.rs | HIGH | Low | 2 tests |
| Accept Offer PDA | marketplace.rs | HIGH | Low | Already blocked by #2 |

### Current Test Status
- **Passing:** 15/18 (83%)
- **Failing due to bugs:** 3/18 (17%)

### Workaround for Testing
Until these bugs are fixed, you can:
1. Skip the failing tests with `.skip()` in Mocha
2. Comment out the failing test cases
3. Deploy with working features and patch later

### Recommended Action Plan
1. **Immediate:** Fix Bug #2 and #3 (simple seed changes)
2. **Short-term:** Implement liquidity vault for Bug #1
3. **Long-term:** Add integration tests during development to catch these early

---

## Additional Notes

### Why These Bugs Weren't Caught Earlier
1. **No integration tests** - Smart contract was implemented without running actual transactions
2. **Anchor build passes** - These are runtime errors, not compilation errors
3. **PDA derivation is subtle** - Easy to mix up different seed combinations

### Impact on Production
- 🟢 **User profiles, tipping, subscriptions, groups, governance all work perfectly**
- 🟡 **Bonding curve buy works, sell doesn't** - Users can get stuck holding shares
- 🔴 **Marketplace offers broken** - Only direct buy/sell works, no negotiations

### Security Implications
Bug #1 (Sell Shares) could be a **critical security issue** if not fixed before mainnet:
- Users buy shares and send SOL to creator
- Users cannot sell shares back
- Creator accumulates SOL with no exit liquidity
- This could be seen as a "honeypot" or rug-pull mechanism

**Recommendation:** Do NOT deploy to mainnet until Bug #1 is fixed with a proper vault.

---

## Testing Methodology

These bugs were discovered through:
1. Comprehensive integration test suite (18 tests)
2. Actual on-chain transactions against local validator
3. Anchor framework error messages
4. Cross-referencing PDA derivations across all instructions
5. Account constraint validation

---

## Next Steps for Development Team

### Priority 1: Fix Make Offer and Accept Offer (30 min)
```bash
# Edit marketplace.rs
# Change listing seeds in MakeOffer and AcceptOffer
# Add username_nft account to MakeOffer if needed
anchor build
anchor test
# Should see 17/18 tests passing
```

### Priority 2: Implement Liquidity Vault (2-4 hours)
```bash
# Create pool_vault PDA in shares.rs
# Update buy_shares to deposit to vault
# Update sell_shares to withdraw from vault
# Add vault balance tracking
anchor build
anchor test
# Should see 18/18 tests passing ✅
```

### Priority 3: Deploy to Devnet
```bash
anchor deploy --provider.cluster devnet
# Test with real SOL
# Monitor transactions
```

---

## Conclusion

These bugs demonstrate the importance of **test-driven development** for smart contracts. While the contract compiles successfully, runtime behavior reveals critical flaws that could cause loss of funds or broken functionality.

**Good news:** All bugs are fixable with localized changes. The overall architecture is sound. 15/18 features work perfectly.

**Action required:** Fix these 3 bugs before any production deployment.
