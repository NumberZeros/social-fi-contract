# Deployment Guide
**Project:** Social-Fi Smart Contract  
**Version:** 1.0.1  
**Security Score:** 8.5/10

---

## Pre-Deployment Checklist

### ✅ Security
- [x] All critical issues resolved (6/6)
- [x] Tests passing (18/18 - 100%)
- [x] Overflow protection implemented
- [x] Slippage controls active
- [x] Emergency pause mechanism ready
- [x] CEI pattern for reentrancy protection
- [ ] External audit (recommended)
- [ ] Bug bounty program
- [ ] Multisig admin wallet setup

### ✅ Configuration
- [ ] Admin wallet configured (use multisig)
- [ ] Fee collector address set
- [ ] Protocol fee rate configured (default: 1%)
- [ ] Pause state = false
- [ ] Base prices set for creator pools

### ✅ Testing
- [x] Unit tests complete
- [x] Integration tests passing
- [ ] Devnet deployment tested
- [ ] Load testing completed
- [ ] Pause/unpause tested
- [ ] Fee collection verified

### ✅ Documentation
- [x] Security audit report
- [x] Deployment guide
- [ ] User documentation
- [ ] API documentation
- [ ] Emergency procedures

---

## Devnet Deployment (1-2 weeks)

### Step 1: Build for Devnet
```bash
# Clean build
anchor clean
anchor build

# Verify program ID
anchor keys list

# Update Anchor.toml and lib.rs with program ID
```

### Step 2: Deploy to Devnet
```bash
# Set Solana config to devnet
solana config set --url https://api.devnet.solana.com

# Airdrop SOL for deployment
solana airdrop 5

# Deploy program
anchor deploy
```

### Step 3: Initialize Platform Config
```bash
# Run initialization script
ts-node scripts/initialize-devnet.ts

# Verify:
# - Admin = your devnet wallet
# - Fee collector = designated address
# - Paused = false
# - Protocol fee = 100 bps (1%)
```

### Step 4: Devnet Testing Plan

**Week 1: Basic Functionality**
- Day 1-2: User profiles, tipping
- Day 3-4: Creator pools, buying/selling shares
- Day 5-6: Subscriptions, group management
- Day 7: Governance, NFT marketplace

**Week 2: Security & Load Testing**
- Test pause/unpause mechanism
- Attempt overflow attacks (should fail)
- Test slippage protection
- High-frequency trading simulation
- Multi-user stress test
- Admin functions (fee withdrawal, config updates)

**Monitoring Checklist:**
- [ ] All transactions successful
- [ ] No unexpected reverts
- [ ] Fees collected correctly
- [ ] Pause works instantly
- [ ] No state corruption
- [ ] Gas costs reasonable (<0.01 SOL)

---

## Mainnet-Beta Deployment

### Step 1: Prepare Mainnet Environment

```bash
# Switch to mainnet-beta
solana config set --url https://api.mainnet-beta.solana.com

# Use hardware wallet or multisig
# DO NOT use hot wallet for admin
```

### Step 2: Security Setup

**Multisig Admin Wallet (Recommended)**
```bash
# Install Squads Protocol or similar
# Create 3-of-5 multisig
# Signers:
# - 2x Core team members
# - 1x Technical lead
# - 1x Community representative
# - 1x External auditor

# Set multisig as admin in initialize_platform
```

**Emergency Contacts**
- Primary: [Your name] - [Phone] - [Email]
- Backup: [Technical lead] - [Phone] - [Email]
- Security: [External auditor] - [Emergency hotline]

### Step 3: Deploy to Mainnet

```bash
# Final build
anchor clean
anchor build --verifiable

# Deploy (requires significant SOL for rent)
anchor deploy

# Cost estimate: ~10-15 SOL for accounts + rent
```

### Step 4: Initialize Platform Configuration

```typescript
// scripts/initialize-mainnet.ts

const adminWallet = MULTISIG_ADDRESS; // Use multisig!
const feeCollector = FEE_COLLECTOR_ADDRESS;
const protocolFeeBps = 100; // 1% (conservative start)

await program.methods
  .initializePlatform()
  .accounts({
    platformConfig: platformConfigPDA,
    admin: adminWallet,
    feeCollector: feeCollector,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

console.log("Platform initialized");
console.log("Admin:", adminWallet.toString());
console.log("Fee Collector:", feeCollector.toString());
```

### Step 5: Gradual Rollout

**Phase 1: Whitelist (Week 1)**
- 50 invited users
- Core features only (profiles, tipping, shares)
- 24/7 monitoring
- Quick pause if needed

**Phase 2: Beta (Weeks 2-3)**
- 500 users
- All features enabled
- Community feedback
- Bug reporting incentives

**Phase 3: Public Launch (Week 4+)**
- Public announcement
- Marketing push
- Full feature set
- Bug bounty active

---

## Monitoring & Alerts

### Key Metrics to Monitor

**Health Metrics:**
- Total Value Locked (TVL)
- Active users (daily/weekly)
- Transaction success rate
- Average gas cost
- Failed transaction reasons

**Security Metrics:**
- Pause events
- Admin actions
- Unusual trading patterns
- Large transfers (>100 SOL)
- Repeated failed attempts

**Financial Metrics:**
- Protocol fees collected
- Pool liquidity levels
- Share price volatility
- Subscription revenue

### Alert Thresholds

| Metric | Warning | Critical | Action |
|--------|---------|----------|--------|
| Failed tx rate | >5% | >10% | Investigate, consider pause |
| Pool liquidity | <20% | <10% | Monitor, alert users |
| Gas spike | >0.02 SOL | >0.05 SOL | Optimize code |
| Admin actions | Any | Any | Log and verify |
| Pause event | - | Any | Emergency response |

### Monitoring Tools

**Blockchain Explorers:**
- Solana Explorer (mainnet-beta)
- Solscan
- SolanaFM

**Custom Monitoring:**
```bash
# Set up cron job to check key metrics
# scripts/monitor.sh

#!/bin/bash
while true; do
  # Check platform config
  anchor run monitor-platform
  
  # Check pool liquidity
  anchor run monitor-pools
  
  # Check for pause
  anchor run check-pause-state
  
  sleep 300 # Every 5 minutes
done
```

**Alerting:**
- Discord webhook for warnings
- PagerDuty for critical alerts
- SMS for admin actions

---

## Emergency Procedures

### Scenario 1: Exploit Detected

**Immediate Actions (0-5 minutes):**
1. Pause platform via admin wallet
   ```bash
   anchor run pause-emergency
   ```
2. Alert core team
3. Begin investigation

**Short-term (5-60 minutes):**
1. Identify attack vector
2. Estimate damage
3. Document exploit
4. Prepare fix

**Medium-term (1-24 hours):**
1. Deploy patched contract
2. Migrate state if needed
3. Compensate affected users
4. Public disclosure

### Scenario 2: High Volume Attack

**Signs:**
- Transaction rate spike (>1000/min)
- Failed transactions surge
- Gas price manipulation

**Response:**
1. Monitor for actual exploit (vs. legitimate traffic)
2. If malicious: pause platform
3. If legitimate: celebrate growth 🎉
4. Scale infrastructure if needed

### Scenario 3: Admin Key Compromise

**Immediate:**
1. Pause platform from backup admin
2. Transfer admin to new multisig
3. Investigate breach

**Recovery:**
1. Rotate all keys
2. Audit all admin actions
3. Revert unauthorized changes
4. Security review

---

## Post-Deployment

### Week 1: Critical Monitoring
- 24/7 on-call rotation
- Check metrics every hour
- User feedback collection
- Bug reports prioritized

### Month 1: Stabilization
- Daily metric reviews
- Weekly team sync
- User interviews
- Feature prioritization

### Month 3+: Growth Phase
- External security audit
- Bug bounty increase ($10k → $50k)
- Version 2 planning (Metaplex integration)
- Marketing expansion

---

## Upgrade Path

### Minor Updates (Bug Fixes)
```bash
# Build new version
anchor build

# Test on devnet
anchor test

# Deploy upgrade
anchor upgrade <PROGRAM_ID> target/deploy/social_fi_contract.so
```

### Major Updates (Breaking Changes)
1. Deploy new program
2. Migration script for state
3. Gradual user migration
4. Deprecate old contract

---

## Contact Information

### Core Team
- **Project Lead:** [Name] - [Email]
- **Technical Lead:** [Name] - [Email]
- **Security Officer:** [Name] - [Email]

### External
- **Auditor:** [Firm name] - [Email]
- **Infrastructure:** [Provider] - [Support]
- **Community:** Discord - Twitter - Telegram

### Emergency Hotline
- **Phone:** [Number]
- **Discord:** @admin-emergency
- **Email:** security@your-project.com

---

## Resources

### Documentation
- [Security Audit](./FINAL_SECURITY_REPORT.md)
- [User Guide](./USER_GUIDE.md)
- [API Docs](./API_DOCUMENTATION.md)

### Tools
- [Squads Multisig](https://squads.so/)
- [Solana Explorer](https://explorer.solana.com/)
- [Anchor Docs](https://www.anchor-lang.com/)

### Community
- Discord: [Link]
- Twitter: [Link]
- GitHub: [Link]

---

**Last Updated:** December 14, 2025  
**Version:** 1.0.1  
**Status:** Ready for Devnet Deployment

---

## Deployment Sign-Off

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Technical Lead | _________ | _________ | __/__/__ |
| Security Officer | _________ | _________ | __/__/__ |
| Project Lead | _________ | _________ | __/__/__ |

**Approved for deployment:** ☐ Yes ☐ No

**Notes:**
_____________________________________________
_____________________________________________
_____________________________________________
