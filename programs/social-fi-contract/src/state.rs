use anchor_lang::prelude::*;
use crate::constants::*;

// ==================== User Profile ====================

#[account]
pub struct UserProfile {
    pub owner: Pubkey,              // 32
    pub username: String,           // 4 + 20 = 24
    pub total_tips_sent: u64,       // 8
    pub total_tips_received: u64,   // 8
    pub posts_count: u64,           // 8
    pub followers_count: u64,       // 8
    pub following_count: u64,       // 8
    pub referral_code: String,      // 4 + 10 = 14
    pub referred_by: Option<Pubkey>, // 1 + 32 = 33
    pub referrals_count: u64,       // 8
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl UserProfile {
    pub const LEN: usize = 8 + 32 + 24 + 8 + 8 + 8 + 8 + 8 + 14 + 33 + 8 + 8 + 1;
}

// ==================== Creator Shares ====================

#[account]
pub struct CreatorPool {
    pub creator: Pubkey,            // 32
    pub supply: u64,                // 8
    pub holders_count: u64,         // 8
    pub base_price: u64,            // 8
    pub total_volume: u64,          // 8
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl CreatorPool {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 1;

    pub fn calculate_price(&self, supply: u64) -> Result<u64> {
        // price = base_price * (supply / PRICE_SCALE)^2
        let supply_scaled = supply
            .checked_div(PRICE_SCALE)
            .ok_or(error!(crate::errors::SocialFiError::BondingCurveOverflow))?;
        
        let price_multiplier = supply_scaled
            .checked_mul(supply_scaled)
            .ok_or(error!(crate::errors::SocialFiError::BondingCurveOverflow))?;
        
        let price = self.base_price
            .checked_mul(price_multiplier)
            .ok_or(error!(crate::errors::SocialFiError::BondingCurveOverflow))?;
        
        Ok(price.max(self.base_price))
    }

    pub fn calculate_buy_cost(&self, amount: u64) -> Result<u64> {
        let mut total_cost: u64 = 0;
        let current_supply = self.supply;
        
        for i in 0..amount {
            let new_supply = current_supply
                .checked_add(i)
                .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?
                .checked_add(1)
                .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?;
            
            let price = self.calculate_price(new_supply)?;
            total_cost = total_cost
                .checked_add(price)
                .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?;
        }
        
        Ok(total_cost)
    }

    pub fn calculate_sell_return(&self, amount: u64) -> Result<u64> {
        let mut total_return: u64 = 0;
        let current_supply = self.supply;
        
        for i in 0..amount {
            let supply_after_sell = current_supply
                .checked_sub(i)
                .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))?;
            
            let price = self.calculate_price(supply_after_sell)?;
            total_return = total_return
                .checked_add(price)
                .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?;
        }
        
        // Apply 10% sell fee
        let fee = total_return
            .checked_mul(SELL_FEE_BPS)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))?;
        
        total_return
            .checked_sub(fee)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))
    }
}

#[account]
pub struct ShareHolding {
    pub holder: Pubkey,             // 32
    pub creator: Pubkey,            // 32
    pub amount: u64,                // 8
    pub average_price: u64,         // 8
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl ShareHolding {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1;
}

// ==================== Subscriptions ====================

#[account]
pub struct SubscriptionTier {
    pub creator: Pubkey,            // 32
    pub tier_id: u64,               // 8
    pub name: String,               // 4 + 20 = 24
    pub description: String,        // 4 + 100 = 104
    pub price: u64,                 // 8 (in lamports)
    pub duration_days: u64,         // 8
    pub subscriber_count: u64,      // 8
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl SubscriptionTier {
    pub const LEN: usize = 8 + 32 + 8 + 24 + 104 + 8 + 8 + 8 + 8 + 1;
}

#[account]
pub struct Subscription {
    pub subscriber: Pubkey,         // 32
    pub creator: Pubkey,            // 32
    pub tier_id: u64,               // 8
    pub start_date: i64,            // 8
    pub end_date: i64,              // 8
    pub status: u8,                 // 1 (0=active, 1=expired, 2=cancelled)
    pub auto_renew: bool,           // 1
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl Subscription {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 1 + 8 + 1;

    pub fn is_active(&self, current_time: i64) -> bool {
        self.status == 0 && current_time < self.end_date
    }
}

// ==================== Groups ====================

#[account]
pub struct Group {
    pub id: Pubkey,                 // 32
    pub name: String,               // 4 + 50 = 54
    pub description: String,        // 4 + 200 = 204
    pub creator: Pubkey,            // 32
    pub privacy: u8,                // 1 (0=public, 1=private, 2=secret)
    pub entry_requirement: u8,      // 1 (0=free, 1=pay_sol, 2=hold_token, 3=hold_nft)
    pub entry_price: Option<u64>,   // 1 + 8 = 9
    pub token_mint: Option<Pubkey>, // 1 + 32 = 33
    pub nft_collection: Option<Pubkey>, // 1 + 32 = 33
    pub member_count: u64,          // 8
    pub post_count: u64,            // 8
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl Group {
    pub const LEN: usize = 8 + 32 + 54 + 204 + 32 + 1 + 1 + 9 + 33 + 33 + 8 + 8 + 8 + 1;
}

#[account]
pub struct GroupMember {
    pub group: Pubkey,              // 32
    pub wallet: Pubkey,             // 32
    pub role: u8,                   // 1 (0=owner, 1=admin, 2=moderator, 3=member)
    pub joined_at: i64,             // 8
    pub banned: bool,               // 1
    pub bump: u8,                   // 1
}

impl GroupMember {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 8 + 1 + 1;

    pub fn can_manage_members(&self) -> bool {
        self.role <= 1 // owner or admin
    }

    pub fn can_moderate(&self) -> bool {
        self.role <= 2 // owner, admin, or moderator
    }
}

// ==================== Governance ====================

#[account]
pub struct StakePosition {
    pub staker: Pubkey,             // 32
    pub amount: u64,                // 8
    pub staked_at: i64,             // 8
    pub lock_period: u64,           // 8 (in days)
    pub unlocks_at: i64,            // 8
    pub rewards: u64,               // 8
    pub voting_power: u64,          // 8
    pub bump: u8,                   // 1
}

impl StakePosition {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1;

    pub fn calculate_voting_power(amount: u64, lock_period: u64) -> Result<u64> {
        let multiplier = match lock_period {
            LOCK_0_DAYS => MULTIPLIER_0_DAYS,
            LOCK_30_DAYS => MULTIPLIER_30_DAYS,
            LOCK_90_DAYS => MULTIPLIER_90_DAYS,
            LOCK_180_DAYS => MULTIPLIER_180_DAYS,
            LOCK_365_DAYS => MULTIPLIER_365_DAYS,
            _ => return Err(error!(crate::errors::SocialFiError::InvalidLockPeriod)),
        };

        amount
            .checked_mul(multiplier)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))
    }

    pub fn calculate_rewards(&self, current_time: i64) -> Result<u64> {
        let time_staked = current_time
            .checked_sub(self.staked_at)
            .ok_or(error!(crate::errors::SocialFiError::InvalidTimestamp))?;
        
        let apy = match self.lock_period {
            LOCK_0_DAYS => APY_0_DAYS,
            LOCK_30_DAYS => APY_30_DAYS,
            LOCK_90_DAYS => APY_90_DAYS,
            LOCK_180_DAYS => APY_180_DAYS,
            LOCK_365_DAYS => APY_365_DAYS,
            _ => APY_0_DAYS,
        };

        // rewards = amount * apy * time_staked / SECONDS_PER_YEAR / BPS_DENOMINATOR
        let rewards = self.amount
            .checked_mul(apy)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?
            .checked_mul(time_staked as u64)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticOverflow))?
            .checked_div(SECONDS_PER_YEAR as u64)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(error!(crate::errors::SocialFiError::ArithmeticUnderflow))?;

        Ok(rewards)
    }

    pub fn is_unlocked(&self, current_time: i64) -> bool {
        current_time >= self.unlocks_at
    }
}

#[account]
pub struct Proposal {
    pub id: Pubkey,                 // 32
    pub proposer: Pubkey,           // 32
    pub title: String,              // 4 + 100 = 104
    pub description: String,        // 4 + 500 = 504
    pub category: u8,               // 1 (0=protocol, 1=treasury, 2=feature, 3=parameter)
    pub status: u8,                 // 1 (0=active, 1=passed, 2=rejected, 3=executed, 4=cancelled)
    pub created_at: i64,            // 8
    pub voting_ends_at: i64,        // 8
    pub execution_delay: i64,       // 8
    pub votes_for: u64,             // 8
    pub votes_against: u64,         // 8
    pub votes_abstain: u64,         // 8
    pub quorum_required: u64,       // 8
    pub executed_at: Option<i64>,   // 1 + 8 = 9
    pub bump: u8,                   // 1
}

impl Proposal {
    pub const LEN: usize = 8 + 32 + 32 + 104 + 504 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 9 + 1;

    pub fn is_active(&self, current_time: i64) -> bool {
        self.status == 0 && current_time < self.voting_ends_at
    }

    pub fn has_passed(&self) -> bool {
        let total_votes = self.votes_for + self.votes_against + self.votes_abstain;
        total_votes >= self.quorum_required && self.votes_for > self.votes_against
    }

    pub fn can_execute(&self, current_time: i64) -> bool {
        self.status == 1 && 
        current_time >= self.voting_ends_at + self.execution_delay &&
        self.executed_at.is_none()
    }
}

#[account]
pub struct Vote {
    pub proposal: Pubkey,           // 32
    pub voter: Pubkey,              // 32
    pub vote_type: u8,              // 1 (0=for, 1=against, 2=abstain)
    pub voting_power: u64,          // 8
    pub voted_at: i64,              // 8
    pub bump: u8,                   // 1
}

impl Vote {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 8 + 8 + 1;
}

// ==================== Marketplace ====================

#[account]
pub struct UsernameNFT {
    pub owner: Pubkey,              // 32
    pub username: String,           // 4 + 20 = 24
    pub verified: bool,             // 1
    pub minted_at: i64,             // 8
    pub bump: u8,                   // 1
}

impl UsernameNFT {
    pub const LEN: usize = 8 + 32 + 24 + 1 + 8 + 1;
}

#[account]
pub struct Listing {
    pub seller: Pubkey,             // 32
    pub username: String,           // 4 + 20 = 24
    pub price: u64,                 // 8
    pub category: u8,               // 1 (0=premium, 1=short, 2=rare, 3=custom)
    pub listed_at: i64,             // 8
    pub expires_at: Option<i64>,    // 1 + 8 = 9
    pub bump: u8,                   // 1
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 24 + 8 + 1 + 8 + 9 + 1;
}

#[account]
pub struct Offer {
    pub listing: Pubkey,            // 32
    pub buyer: Pubkey,              // 32
    pub amount: u64,                // 8
    pub created_at: i64,            // 8
    pub expires_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1;

    pub fn is_expired(&self, current_time: i64) -> bool {
        current_time >= self.expires_at
    }
}
