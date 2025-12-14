# Deployment Guide

## Pre-Deployment Checklist

- [ ] All tests pass: `pnpm test`
- [ ] Code linting passes: `pnpm lint`
- [ ] Contract builds successfully: `pnpm build`
- [ ] Security audit completed (mainnet only)
- [ ] Documentation updated
- [ ] Gas optimization reviewed

## Deployment Steps

### 1. Prepare Environment

Update `Anchor.toml` with target cluster:

```toml
[provider]
cluster = "devnet"  # or "mainnet-beta"
wallet = "~/.config/solana/id.json"
```

### 2. Verify Configuration

```bash
solana config get
anchor config get
```

### 3. Airdrop SOL (devnet/testnet)

```bash
# Request SOL airdrop
solana airdrop 2

# Check balance
solana balance
```

### 4. Deploy Contract

```bash
# Deploy to configured cluster
anchor deploy

# Or specify cluster
anchor deploy --provider.cluster devnet
```

### 5. Verify Deployment

```bash
# Check program account
solana program show 8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP

# View transaction
solana transaction-history
```

## Environment-Specific Deployment

### Localnet

```bash
# Start validator
solana-test-validator

# In another terminal, deploy
anchor deploy
```

### Devnet

```bash
# Configure for devnet
anchor deploy --provider.cluster devnet

# Or update Anchor.toml and run
anchor deploy
```

### Mainnet-Beta

⚠️ **CAUTION: Production deployment!**

```bash
# IMPORTANT: Verify everything before deploying
# Ensure sufficient SOL balance
solana balance

# Deploy to mainnet
anchor deploy --provider.cluster mainnet-beta
```

## Post-Deployment

1. **Verify Contract**
   ```bash
   solana program show YOUR_PROGRAM_ID
   ```

2. **Update Program IDs**
   - Update `Anchor.toml`
   - Update frontend configuration
   - Update documentation

3. **Monitor**
   - Watch for errors in logs
   - Monitor transaction costs
   - Track program account storage

4. **Document**
   - Record deployment timestamp
   - Note transaction signatures
   - Update version numbers

## Rollback Plan

In case of issues:

1. Identify the problem
2. Prepare a fix
3. Build new version
4. Deploy upgrade
5. Verify functionality

## Upgrade Procedures

For upgrades, ensure:
- All data migrations are tested
- Backward compatibility is maintained
- State is properly preserved
- Users are notified in advance

## Cost Estimation

### Deployment Costs
- Program deployment: ~0.5 SOL (rent-exempt)
- Transaction fees: ~0.00025 SOL

### Runtime Costs
- Per transaction: ~0.00025 SOL
- Storage: depends on account size

## Troubleshooting

### Common Issues

**"Insufficient balance"**
```bash
# Check balance
solana balance

# Airdrop (devnet/testnet only)
solana airdrop 2
```

**"Program already exists"**
- Use different keypair
- Or upgrade existing program

**"Transaction failed"**
- Check wallet has sufficient balance
- Verify network connectivity
- Review contract logs

## References

- [Anchor Deployment Guide](https://docs.anchor-lang.com/getting-started/deployment)
- [Solana Program Deployment](https://docs.solana.com/cli/deploy-a-program)
- [Program Upgrades](https://docs.solana.com/learn/programs#program-upgrades)
