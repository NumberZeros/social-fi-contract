use anchor_lang::prelude::*;

#[event]
pub struct UserInitialized {
    pub user: Pubkey,
    pub username: String,
    pub timestamp: i64,
}

#[event]
pub struct TipSent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct SharesPurchased {
    pub buyer: Pubkey,
    pub creator: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub total_cost: u64,
    pub timestamp: i64,
}

#[event]
pub struct SharesSold {
    pub seller: Pubkey,
    pub creator: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub total_received: u64,
    pub fee: u64,
    pub timestamp: i64,
}

#[event]
pub struct SubscriptionTierCreated {
    pub creator: Pubkey,
    pub tier_id: u64,
    pub name: String,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct UserSubscribed {
    pub subscriber: Pubkey,
    pub creator: Pubkey,
    pub tier_id: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub timestamp: i64,
}

#[event]
pub struct SubscriptionCancelled {
    pub subscriber: Pubkey,
    pub creator: Pubkey,
    pub tier_id: u64,
    pub timestamp: i64,
}

#[event]
pub struct GroupCreated {
    pub group: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub privacy: u8,
    pub timestamp: i64,
}

#[event]
pub struct MemberJoined {
    pub group: Pubkey,
    pub member: Pubkey,
    pub role: u8,
    pub timestamp: i64,
}

#[event]
pub struct MemberRoleUpdated {
    pub group: Pubkey,
    pub member: Pubkey,
    pub old_role: u8,
    pub new_role: u8,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MemberKicked {
    pub group: Pubkey,
    pub member: Pubkey,
    pub kicked_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensStaked {
    pub staker: Pubkey,
    pub amount: u64,
    pub lock_period: u64,
    pub voting_power: u64,
    pub unlocks_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct TokensUnstaked {
    pub staker: Pubkey,
    pub amount: u64,
    pub rewards: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProposalCreated {
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub category: u8,
    pub voting_ends_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct VoteCast {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_type: u8,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProposalExecuted {
    pub proposal: Pubkey,
    pub executor: Pubkey,
    pub votes_for: u64,
    pub votes_against: u64,
    pub timestamp: i64,
}

#[event]
pub struct UsernameMinted {
    pub owner: Pubkey,
    pub username: String,
    pub nft: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct UsernameListed {
    pub seller: Pubkey,
    pub username: String,
    pub price: u64,
    pub listing: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct UsernameSold {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub username: String,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct OfferMade {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub listing: Pubkey,
    pub amount: u64,
    pub expires_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct OfferAccepted {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub listing: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
