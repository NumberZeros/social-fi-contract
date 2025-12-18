# NFT Metadata Limits

## Metaplex Token Metadata Standard

### Field Limits

| Field | Max Length | Notes |
|-------|-----------|-------|
| **Name** | 32 bytes | NFT title/name |
| **Symbol** | 10 bytes | Token symbol (we use "POST") |
| **URI** | 200 bytes | Off-chain metadata URI |
| **Seller Fee Basis Points** | u16 | 0-10000 (0% - 100%) |

### Our Implementation

**Post Struct:**
```rust
pub struct Post {
    pub author: Pubkey,        // 32 bytes
    pub uri: String,           // 4 + 200 = 204 bytes
    pub nonce: String,         // 4 + 16 = 20 bytes  
    pub mint: Option<Pubkey>,  // 1 + 32 = 33 bytes
    pub created_at: i64,       // 8 bytes
    pub bump: u8,              // 1 byte
}
```

**Validation Constants:**
```rust
pub const MAX_TITLE_LENGTH: usize = 32;        // Metaplex NFT name limit
pub const MAX_DESCRIPTION_LENGTH: usize = 500; // Off-chain metadata
pub const MAX_USERNAME_LENGTH: usize = 20;
pub const MAX_NAME_LENGTH: usize = 50;
```

### Frontend Validation

**MintPostModal.tsx:**
- Title input: `maxLength={32}`
- Character counter: `{title.length}/32`
- Visual warning when exceeding limit

**useMintPost.ts:**
```typescript
if (title.length > 32) {
  toast.error('NFT title must be 32 characters or less');
  return null;
}
```

### Error Codes

| Error Code | Error Name | Description |
|------------|-----------|-------------|
| `0xb` (11) | NameTooLong | NFT name exceeds 32 bytes |
| `0xc` (12) | SymbolTooLong | Symbol exceeds 10 bytes |
| `0xd` (13) | UriTooLong | URI exceeds 200 bytes |

### Best Practices

✅ **Do:**
- Keep NFT titles under 32 characters
- Use descriptive but concise names
- Validate on both frontend and backend
- Show character counter to users

❌ **Don't:**
- Use full post content as NFT name
- Include emojis (count as multiple bytes in UTF-8)
- Exceed Metaplex limits

### Testing

```bash
# Valid title (32 chars)
"My Amazing Web3 Social Post!"

# Invalid title (too long)
"This is an extremely long title that exceeds the Metaplex 32 byte limit"
```

### References

- [Metaplex Token Metadata Standard](https://docs.metaplex.com/programs/token-metadata/overview)
- [Solana Program Library](https://spl.solana.com/)
