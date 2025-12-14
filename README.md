# Social-Fi Smart Contract

> Solana smart contract for decentralized social networks with built-in creator economy.

Imagine Twitter/X where creators own their audience, fans can invest in their favorites, and all transactions are transparent on-chain. This contract makes it possible.

## The Problem

**Web2 social platforms today:**
- 🔒 Platform owns your audience, not you
- 💰 No way to financially support creators you believe in early
- 🎭 Opaque monetization controlled by algorithms
- 📉 Creators can't build equity in their personal brand

**This contract enables:**
- 📈 **Creator Shares** - Fans buy/sell creator shares with bonding curve pricing (early supporters win!)
- 🎯 **True Ownership** - Your profile and social graph live on-chain
- 💸 **Direct Monetization** - Tips, subscriptions, groups without middlemen
- 🗳️ **Community Governance** - Token holders vote on platform decisions

## Quick Example

```typescript
// Invest in a creator early (bonding curve = price goes up as demand increases)
await program.methods
  .buyShares(creatorPubkey, amount, maxPrice)
  .accounts({ buyer, creator, poolVault })
  .rpc();

// Sell shares when creator blows up
await program.methods
  .sellShares(creatorPubkey, amount, minPrice)
  .accounts({ seller, creator, poolVault })
  .rpc();

// Tip creators directly (0% platform fee)
await program.methods
  .sendTip(creatorPubkey, tipAmount, message)
  .accounts({ tipper, creator })
  .rpc();
```

**Real-world use case:** Imagine finding the next Mr. Beast when they only have 1000 followers. Buy their shares at $0.01. When they hit 1M followers, shares are worth $10. Early supporters are rewarded for believing in them.

## Features

| Feature | Description |
|---------|-------------|
| **Creator Shares** | Quadratic bonding curve pricing, auto liquidity pool |
| **Subscriptions** | Recurring payments, auto-renewal, tier support |
| **Groups** | Public/private communities with admin/mod roles |
| **Governance** | Stake tokens, create proposals, vote on changes |
| **NFT Usernames** | Mint, trade, and truly own your handle |
| **Tipping** | P2P payments with optional messages |

## Getting Started

### Install

```bash
# Prerequisites: Rust, Solana CLI v1.18+, Anchor v0.32.1, Node.js v16+
git clone <your-repo>
cd social-fi-contract
make install
```

### Build & Test

```bash
make build        # Compile contract
make test         # Run 18 tests (100% passing)
```

### Deploy

```bash
# Local development
make validator         # Terminal 1: Start validator
make deploy-local      # Terminal 2: Deploy

# Devnet
make deploy-devnet

# Mainnet (use with caution)
make deploy-mainnet
```

## Project Structure

```
programs/social-fi-contract/src/
├── lib.rs              # Program entry (28 instructions)
├── state.rs            # Account structures
├── errors.rs           # Custom errors
└── instructions/
    ├── shares.rs       # Bonding curve logic
    ├── user.rs         # Profiles & tipping
    ├── subscription.rs # Recurring payments
    ├── group.rs        # Community management
    ├── governance.rs   # Voting system
    ├── marketplace.rs  # NFT trading
    └── platform.rs     # Admin controls
```

## Key Concepts

### Bonding Curve

Shares use quadratic pricing: `price = supply²`

- **Early entry advantage:** 1st share = $0.01, 100th share = $1.00, 1000th share = $10.00
- **Built-in liquidity:** All trades go through the pool
- **No order books:** Price is algorithmic, always available

### Security Features

- ✅ Overflow protection (checked math, u128 arithmetic)
- ✅ Slippage protection (max/min price params)
- ✅ Reentrancy guards (CEI pattern)
- ✅ Emergency pause (admin can halt trading)
- ✅ Liquidity protection (configurable minimum pool balance)

## Documentation

- **[API Reference](./docs/API_REFERENCE.md)** - All 28 instructions with examples
- **[Architecture](./docs/ARCHITECTURE.md)** - System design & data flow
- **[Deployment Guide](./docs/DEPLOYMENT_GUIDE.md)** - Production checklist
- **[Security Report](./docs/FINAL_SECURITY_REPORT.md)** - Audit results (9.2/10 score)

## Development

```bash
make help             # Show all 50+ commands
make test             # Run tests
make lint             # Check code style
make format           # Format code
make audit            # Security audit
make logs             # View program logs
make program-id       # Show deployed program ID
```

## Contributing

We welcome contributions! Here's how:

1. Fork the repo
2. Create a branch: `git checkout -b feature/cool-feature`
3. Make changes & test: `make test`
4. Commit: `git commit -m 'Add cool feature'`
5. Push & open a PR

See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for detailed guidelines.

## Testing

18/18 tests passing (100% coverage):
- ✅ User profiles & tipping (2)
- ✅ Creator shares bonding curve (3)
- ✅ Subscription system (3)
- ✅ Group management (3)
- ✅ Governance & voting (3)
- ✅ NFT marketplace (4)

```bash
make test             # Run all tests
make test-watch       # Watch mode
```

## Roadmap

- [x] Core features (profiles, shares, subscriptions)
- [x] Governance system
- [x] NFT marketplace
- [x] Security audit (internal)
- [ ] External audit
- [ ] Mainnet deployment
- [ ] SDKs (TypeScript, Rust)
- [ ] Example frontend app
- [ ] Bug bounty program

## FAQ

**Q: What's a bonding curve?**  
A: Algorithmic pricing where price increases with supply. Early buyers pay less, late buyers pay more. No order books needed.

**Q: Can I lose money on creator shares?**  
A: Yes. If a creator loses popularity, share price drops. This is speculative - only invest what you can afford to lose.

**Q: What happens if the contract is paused?**  
A: Admin can pause trading during emergencies. Your shares are still yours, but you can't trade until unpaused.

**Q: Is this audited?**  
A: Internal audit complete (9.2/10). External audit recommended before mainnet launch.

**Q: Can I use this in my app?**  
A: Yes! Open source under ISC license. See [LICENSE](./LICENSE).

## License

ISC License - see [LICENSE](./LICENSE) file.

## Support

- 🐛 **Bug reports:** [GitHub Issues](https://github.com/your-org/social-fi-contract/issues)
- 💬 **Questions:** [GitHub Discussions](https://github.com/your-org/social-fi-contract/discussions)
- 📖 **Docs:** [Documentation](./docs/)

---

**Built for the Solana community** | [Anchor](https://www.anchor-lang.com/) | [Solana](https://solana.com/)
