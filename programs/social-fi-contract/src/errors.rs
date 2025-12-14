use anchor_lang::prelude::*;

#[error_code]
pub enum SocialFiError {
    #[msg("Username too long (max 20 characters)")]
    UsernameTooLong,
    
    #[msg("Username already taken")]
    UsernameAlreadyTaken,
    
    #[msg("Invalid username format")]
    InvalidUsername,
    
    #[msg("Insufficient balance for this operation")]
    InsufficientBalance,
    
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    
    #[msg("Cannot tip yourself")]
    CannotTipSelf,
    
    #[msg("Bonding curve calculation overflow")]
    BondingCurveOverflow,
    
    #[msg("Not enough shares to sell")]
    InsufficientShares,
    
    #[msg("Subscription tier not found")]
    SubscriptionTierNotFound,
    
    #[msg("Subscription already active")]
    SubscriptionAlreadyActive,
    
    #[msg("Subscription expired or cancelled")]
    SubscriptionInactive,
    
    #[msg("Group name too long")]
    GroupNameTooLong,
    
    #[msg("Invalid group privacy setting")]
    InvalidGroupPrivacy,
    
    #[msg("Invalid entry requirement")]
    InvalidEntryRequirement,
    
    #[msg("Insufficient tokens to join group")]
    InsufficientTokensForGroup,
    
    #[msg("Not a member of this group")]
    NotGroupMember,
    
    #[msg("Insufficient permissions")]
    InsufficientPermissions,
    
    #[msg("Cannot perform action on yourself")]
    CannotActOnSelf,
    
    #[msg("Member already in group")]
    MemberAlreadyInGroup,
    
    #[msg("Member is banned from this group")]
    MemberBanned,
    
    #[msg("Invalid lock period")]
    InvalidLockPeriod,
    
    #[msg("Tokens are still locked")]
    TokensLocked,
    
    #[msg("Insufficient voting power to create proposal")]
    InsufficientVotingPower,
    
    #[msg("Proposal title too long")]
    ProposalTitleTooLong,
    
    #[msg("Proposal description too long")]
    ProposalDescriptionTooLong,
    
    #[msg("Invalid proposal category")]
    InvalidProposalCategory,
    
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    
    #[msg("Voting period still active")]
    VotingPeriodActive,
    
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
    
    #[msg("Quorum not reached")]
    QuorumNotReached,
    
    #[msg("Proposal did not pass")]
    ProposalNotPassed,
    
    #[msg("Execution delay not met")]
    ExecutionDelayNotMet,
    
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Username NFT already minted")]
    UsernameAlreadyMinted,
    
    #[msg("Not the owner of this username NFT")]
    NotUsernameOwner,
    
    #[msg("Username listing not found")]
    ListingNotFound,
    
    #[msg("Listing price must be greater than zero")]
    InvalidListingPrice,
    
    #[msg("Offer amount must be greater than zero")]
    InvalidOfferAmount,
    
    #[msg("Offer not found or expired")]
    OfferNotFound,
    
    #[msg("Not the listing seller")]
    NotListingSeller,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
    
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    
    #[msg("Supply exceeds maximum limit")]
    SupplyTooHigh,
    
    #[msg("Price exceeds maximum limit")]
    PriceTooHigh,
    
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    
    #[msg("Contract is paused")]
    ContractPaused,
    
    #[msg("Unauthorized: admin only")]
    Unauthorized,
    
    #[msg("Reentrancy detected")]
    Reentrancy,
    
    #[msg("Minimum liquidity requirement not met")]
    MinimumLiquidityRequired,
    
    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity,
}
