# Migrations

Migration scripts cho Social-Fi contract deployment và setup.

## Available Scripts

### 1. Initialize Platform
Initialize platform config (chỉ chạy 1 lần sau deploy):
```bash
pnpm migration:init
```

### 2. Verify Deployment
Kiểm tra program và platform config:
```bash
pnpm migration:verify
```

### 3. Deploy
Full deployment script:
```bash
pnpm migration:deploy
```

## Program Information

**Current Program ID:** `FHHfGX8mYxagDmhsXgJUfLnx1rw2M138e3beCwWELdgL`

**Platform Config PDA:** `HBcvVewwYEwjDLzvu3MVDXGUq8dpn5mhSeeqbRQiDQEs`

**Network:** Devnet

## Deployment History

### v0.2.1 - Dec 18, 2025 (Current)
- **Program ID:** FHHfGX8mYxagDmhsXgJUfLnx1rw2M138e3beCwWELdgL
- **Changes:**
  - Fixed MAX_TITLE_LENGTH: 100 → 32 (Metaplex limit)
  - Added NFT metadata validation
- **Deploy Signature:** 59TPLjTSWRVYQ7ihAzdwbT7sF8XrPPt5qhdb1dCkePK5DmWsC8xx7f53oX7o2vxS7a8epBEx5r6VpqyvnXAJEnky

### v0.2.0 - Dec 18, 2025
- **Program ID:** FHHfGX8mYxagDmhsXgJUfLnx1rw2M138e3beCwWELdgL
- **Changes:**
  - Added `nonce` field to Post struct
  - Fixed CPI signer seeds mismatch in mintPost
  - Seeds now use stored nonce instead of uri
- **Deploy Signature:** 5ETm16Aq7YiqxcbutXoE2Ne85pKWUi9nyNPQ7n2iaRPurZX9CdKdSEvxM5b7Wzktxkv6GMh7U83u2QxwGFs1GYPH
- **Platform Init:** 2hHeK3BfFeYpvLxYDJ3qEyDugf6tb5uyCQGKHsTeiYqtovVKkPQu3eLcMtLfe9bWmj45xgL4ofiANQKQLienwr5Q

### v0.1.0 - Dec 15, 2025 (Deprecated)
- **Program ID:** 8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP
- **Status:** ⚠️ Has critical bug - seeds mismatch in mintPost
- **Note:** Cannot close (different authority)

## Manual Deployment Steps

If you need to deploy manually:

```bash
# 1. Build
anchor build

# 2. Deploy
solana program deploy target/deploy/social_fi_contract.so

# 3. Update program ID in code
# - programs/social-fi-contract/src/lib.rs (declare_id!)
# - Anchor.toml

# 4. Rebuild with new ID
anchor build

# 5. Redeploy
solana program deploy --program-id <keypair.json> target/deploy/social_fi_contract.so

# 6. Initialize platform
pnpm migration:init

# 7. Verify
pnpm migration:verify
```

## Important Notes

⚠️ **Always verify deployment:**
```bash
pnpm migration:verify
```

⚠️ **Before deploying to mainnet:**
- Test thoroughly on devnet
- Audit smart contract code
- Have sufficient SOL for deployment (~7-10 SOL)
- Backup all keypairs

⚠️ **Program upgrades:**
- Upgrade authority is the deployer wallet
- Keep upgrade authority keypair safe
- Use `solana program set-upgrade-authority` to transfer if needed
