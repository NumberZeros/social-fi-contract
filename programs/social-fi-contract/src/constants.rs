// PDA Seeds
pub const USER_PROFILE_SEED: &[u8] = b"user_profile";
pub const CREATOR_POOL_SEED: &[u8] = b"creator_pool";
pub const SHARE_HOLDING_SEED: &[u8] = b"share_holding";
pub const SUBSCRIPTION_TIER_SEED: &[u8] = b"subscription_tier";
pub const SUBSCRIPTION_SEED: &[u8] = b"subscription";
pub const GROUP_SEED: &[u8] = b"group";
pub const GROUP_MEMBER_SEED: &[u8] = b"group_member";
pub const STAKE_POSITION_SEED: &[u8] = b"stake_position";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const VOTE_SEED: &[u8] = b"vote";
pub const USERNAME_NFT_SEED: &[u8] = b"username_nft";
pub const LISTING_SEED: &[u8] = b"listing";
pub const OFFER_SEED: &[u8] = b"offer";
pub const POST_SEED: &[u8] = b"post";
pub const FOLLOW_SEED: &[u8] = b"follow";
pub const LIKE_SEED: &[u8] = b"like";
pub const REPOST_SEED: &[u8] = b"repost";
pub const COMMENT_SEED: &[u8] = b"comment";

// Bonding Curve Constants
pub const BASE_PRICE: u64 = 10_000_000; // 0.01 SOL in lamports
pub const PRICE_SCALE: u64 = 100; // Scale factor for bonding curve
pub const SELL_FEE_BPS: u64 = 1000; // 10% in basis points
pub const MAX_SUPPLY: u64 = 1_000_000; // Maximum supply to prevent overflow
pub const MAX_PRICE: u64 = u64::MAX / 1000; // Max price cap

// Governance Constants
pub const MIN_VOTING_POWER: u64 = 1000;
pub const QUORUM_BPS: u64 = 1000; // 10% in basis points
pub const VOTING_PERIOD: i64 = 7 * 24 * 60 * 60; // 7 days in seconds
pub const MIN_EXECUTION_DELAY: i64 = 24 * 60 * 60; // 24 hours

// Lock Period Constants (in days)
pub const LOCK_0_DAYS: u64 = 0;
pub const LOCK_30_DAYS: u64 = 30;
pub const LOCK_90_DAYS: u64 = 90;
pub const LOCK_180_DAYS: u64 = 180;
pub const LOCK_365_DAYS: u64 = 365;

// Voting Power Multipliers (in basis points)
pub const MULTIPLIER_0_DAYS: u64 = 10000; // 1.0x
pub const MULTIPLIER_30_DAYS: u64 = 12000; // 1.2x
pub const MULTIPLIER_90_DAYS: u64 = 15000; // 1.5x
pub const MULTIPLIER_180_DAYS: u64 = 20000; // 2.0x
pub const MULTIPLIER_365_DAYS: u64 = 30000; // 3.0x

// APY Rates (in basis points)
pub const APY_0_DAYS: u64 = 500; // 5%
pub const APY_30_DAYS: u64 = 1000; // 10%
pub const APY_90_DAYS: u64 = 1500; // 15%
pub const APY_180_DAYS: u64 = 2000; // 20%
pub const APY_365_DAYS: u64 = 3000; // 30%

// String Limits
pub const MAX_USERNAME_LENGTH: usize = 20;
pub const MAX_TITLE_LENGTH: usize = 32; // Metaplex NFT name limit
pub const MAX_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_NAME_LENGTH: usize = 50;

// Marketplace Constants
pub const MIN_OFFER_AMOUNT: u64 = 100_000; // 0.0001 SOL minimum offer

// Time Constants
pub const SECONDS_PER_DAY: i64 = 86400;
pub const SECONDS_PER_YEAR: i64 = 31536000;

// Precision
pub const BPS_DENOMINATOR: u64 = 10000;
