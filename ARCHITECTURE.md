# Social Fi Contract Architecture

## Overview

Social Fi Contract is a Solana smart contract built with Anchor that enables social finance functionality on the blockchain.

## Project Structure

```
social-fi-contract/
├── programs/
│   └── social-fi-contract/      # Main contract code
│       ├── src/
│       │   └── lib.rs            # Contract entry point
│       └── Cargo.toml            # Rust dependencies
├── tests/                         # Integration tests
├── migrations/                    # Deployment scripts
├── Anchor.toml                   # Anchor configuration
└── package.json                  # Node.js configuration
```

## Smart Contract Components

### Program Structure

```rust
pub mod social_fi_contract {
    // Program logic
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Implementation
    }
}
```

### Account Structures

Currently implements:
- `Initialize` - Basic initialization context

Future additions:
- User accounts
- Token accounts
- Pool accounts
- Governance accounts

## Development Workflow

### 1. Local Development

```bash
# Start local validator
solana-test-validator

# In another terminal
anchor test
```

### 2. Testing

Write tests in `tests/social-fi-contract.ts`:

```typescript
describe("social-fi-contract", () => {
  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Transaction signature", tx);
  });
});
```

### 3. Building

```bash
anchor build
```

Generates:
- Compiled contract: `target/deploy/`
- IDL: `target/idl/`

### 4. Deployment

See [DEPLOYMENT.md](./DEPLOYMENT.md)

## Solana Concepts

### Program Accounts

- **Executable**: Smart contract code
- **Data**: Account that stores program state
- **Signer**: Account that pays for transactions

### Program ID

```
8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP
```

### Transactions

```typescript
const tx = await program.methods
  .instructionName(args)
  .accounts({
    // required accounts
  })
  .rpc();
```

## Development Guidelines

### Anchor Best Practices

1. **Use Constraints**
   ```rust
   #[account(init, payer = user, space = 8 + 32)]
   pub data: Account<'info, MyData>,
   ```

2. **Validate Inputs**
   ```rust
   require!(amount > 0, ErrorCode::InvalidAmount);
   ```

3. **Use Events**
   ```rust
   emit!(MyEvent { /* fields */ });
   ```

4. **Proper Error Handling**
   ```rust
   pub enum ErrorCode {
       #[msg("Invalid amount")]
       InvalidAmount,
   }
   ```

### Testing

- Write unit tests in Rust
- Write integration tests in TypeScript
- Test all error conditions
- Use fixtures for test setup

## Dependencies

### Rust (Smart Contract)
- `anchor-lang` - Anchor framework
- Solana runtime libraries

### Node.js (Testing)
- `@coral-xyz/anchor` - Anchor client library
- `@solana/web3.js` - Solana web3 library
- `ts-mocha` - TypeScript test runner

## Security Considerations

1. **Input Validation** - Always validate instruction arguments
2. **Access Control** - Verify account ownership and permissions
3. **Rent Exemption** - Ensure accounts are rent-exempt
4. **Re-entrancy** - Be aware of cross-contract calls

## Performance Optimization

- Minimize account allocations
- Use efficient data structures
- Batch operations when possible
- Consider compute unit costs

## Monitoring & Debugging

### View Logs

```bash
# Show recent logs
solana logs

# Follow live logs
solana logs --follow
```

### Debug Information

```rust
msg!("Debug message: {}", value);
```

## Version Management

Current version: `0.1.0`

Breaking changes documented in [CHANGELOG.md](./CHANGELOG.md)

## References

- [Anchor Documentation](https://docs.anchor-lang.com/)
- [Solana Program Library](https://github.com/solana-labs/solana-program-library)
- [Solana Cookbook - Anchor](https://solanacookbook.com/references/anchor.html)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md)
