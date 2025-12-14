# Deployment Information

## Devnet Deployment

**Date:** December 14, 2025  
**Status:** ✅ LIVE on Devnet

### Program Details

- **Program ID:** `8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP`
- **IDL Account:** `AogVde71ib9bSs6Dnfgo1NHXMxtT9NB6NApzQ8uX6xEu`
- **Upgrade Authority:** `HMj2bYhaCsyJvybwwJSiufTK9AqDmuLrwhQgy2wvWdfi`
- **Deployed Slot:** 428225712
- **Binary Size:** 707,360 bytes (~691 KB)
- **Network:** Devnet

### Explorer Links

- **Program:** https://explorer.solana.com/address/8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP?cluster=devnet
- **IDL:** https://explorer.solana.com/address/AogVde71ib9bSs6Dnfgo1NHXMxtT9NB6NApzQ8uX6xEu?cluster=devnet

### How to Connect

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SocialFiContract } from "../target/types/social_fi_contract";

const connection = new anchor.web3.Connection(
  "https://api.devnet.solana.com",
  "confirmed"
);

const programId = new anchor.web3.PublicKey(
  "8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP"
);

const program = new Program<SocialFiContract>(idl, programId, provider);
```

### Test on Devnet

```bash
# Set cluster to devnet
solana config set --url devnet

# Check program info
solana program show 8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP

# View logs
solana logs 8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP
```

### Next Steps

- [ ] Test all instructions on devnet
- [ ] Monitor for 1-2 weeks
- [ ] Collect user feedback
- [ ] Fix any bugs discovered
- [ ] External security audit
- [ ] Deploy to mainnet

### Upgrade Command

```bash
# Build new version
make build-release

# Upgrade on devnet
make upgrade-devnet
```

---

**Last Updated:** December 14, 2025
