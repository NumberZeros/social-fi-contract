# Social Fi Contract

A Solana smart contract for Social Finance (Social Fi) using Anchor framework.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [pnpm](https://pnpm.io/installation)

### Check Installations

```bash
rustc --version
solana --version
anchor --version
node --version
pnpm --version
```

## Project Structure

```
.
├── programs/
│   └── social-fi-contract/
│       └── src/
│           └── lib.rs          # Main smart contract code
├── tests/
│   └── social-fi-contract.ts   # Integration tests
├── migrations/
│   └── deploy.ts               # Deployment script
└── Anchor.toml                 # Anchor configuration
```

## Setup

1. **Install dependencies:**

```bash
pnpm install
```

2. **Configure Solana CLI:**

```bash
# Set network cluster (localnet, devnet, mainnet-beta)
solana config set --url localhost

# Set wallet (keypair)
solana config set --keypair ~/.config/solana/id.json
```

3. **Create a local wallet if needed:**

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

4. **Check configuration:**

```bash
solana config get
```

## Development

### Build

```bash
anchor build
```

### Test

Run all tests:

```bash
anchor test
```

Run specific test file:

```bash
pnpm ts-mocha -p ./tsconfig.json -t 1000000 "tests/social-fi-contract.ts"
```

### Linting

Check code style:

```bash
pnpm lint
```

Fix code style issues:

```bash
pnpm lint:fix
```

## Deployment

### Local Network (Localnet)

1. **Start local Solana validator:**

```bash
solana-test-validator
```

2. **Deploy to localnet:**

```bash
anchor deploy
```

### Devnet

```bash
# Update Anchor.toml cluster to devnet
anchor deploy --provider.cluster devnet
```

### Mainnet

```bash
# Update Anchor.toml cluster to mainnet-beta
anchor deploy --provider.cluster mainnet-beta
```

## CLI Commands

```bash
# Build the contract
pnpm build

# Test the contract
pnpm test

# Deploy the contract
pnpm deploy

# Generate IDL
pnpm idl

# Format code
pnpm lint:fix
```

## Cluster Management

### View Account Balance

```bash
solana balance
```

### View Recent Transactions

```bash
solana transaction-history
```

### Airdrop SOL (devnet/testnet only)

```bash
solana airdrop 2
```

## Troubleshooting

### Common Issues

1. **"Wallet not found"**
   - Ensure wallet path in `Anchor.toml` is correct
   - Create wallet: `solana-keygen new --outfile ~/.config/solana/id.json`

2. **"Program not deployed"**
   - Run `anchor build` before `anchor deploy`
   - Ensure you have enough SOL for deployment

3. **"Connection refused"**
   - Start local validator: `solana-test-validator`
   - Or configure correct RPC endpoint

4. **"Insufficient balance"**
   - Airdrop SOL: `solana airdrop 2` (devnet/testnet only)
   - For mainnet, you need to fund from an exchange

## References

- [Anchor Documentation](https://docs.anchor-lang.com/)
- [Solana Developer Documentation](https://docs.solana.com/developers)
- [Solana Cookbook](https://solanacookbook.com/)

## License

ISC
