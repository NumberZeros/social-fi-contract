use anchor_lang::prelude::*;

declare_id!("8dU8UsnavCaqmm4JTgMHCtjzcfcu4D4iKYW71MXE1mDP");

pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod constants;

use instructions::*;

#[program]
pub mod social_fi_contract {
    use super::*;

    // ==================== Platform Management ====================
    
    pub fn initialize_platform(ctx: Context<InitializePlatform>, fee_collector: Pubkey) -> Result<()> {
        instructions::platform::initialize_platform(ctx, fee_collector)
    }
    
    pub fn pause_platform(ctx: Context<UpdatePlatform>) -> Result<()> {
        instructions::platform::pause_platform(ctx)
    }
    
    pub fn unpause_platform(ctx: Context<UpdatePlatform>) -> Result<()> {
        instructions::platform::unpause_platform(ctx)
    }
    
    pub fn update_admin(ctx: Context<UpdatePlatform>, new_admin: Pubkey) -> Result<()> {
        instructions::platform::update_admin(ctx, new_admin)
    }
    
    pub fn update_fee_collector(ctx: Context<UpdatePlatform>, new_fee_collector: Pubkey) -> Result<()> {
        instructions::platform::update_fee_collector(ctx, new_fee_collector)
    }
    
    pub fn update_min_liquidity(ctx: Context<UpdatePlatform>, new_min_liquidity_bps: u64) -> Result<()> {
        instructions::platform::update_min_liquidity(ctx, new_min_liquidity_bps)
    }

    // ==================== User & Tipping ====================
    
    pub fn initialize_user(ctx: Context<InitializeUser>, username: String) -> Result<()> {
        instructions::user::initialize_user(ctx, username)
    }

    pub fn send_tip(ctx: Context<SendTip>, amount: u64) -> Result<()> {
        instructions::user::send_tip(ctx, amount)
    }

    // ==================== Creator Shares (Bonding Curve) ====================
    
    pub fn initialize_creator_pool(ctx: Context<InitializeCreatorPool>) -> Result<()> {
        instructions::shares::initialize_creator_pool(ctx)
    }

    pub fn buy_shares(ctx: Context<BuyShares>, amount: u64, max_price_per_share: u64) -> Result<()> {
        instructions::shares::buy_shares(ctx, amount, max_price_per_share)
    }

    pub fn sell_shares(ctx: Context<SellShares>, amount: u64, min_price_per_share: u64) -> Result<()> {
        instructions::shares::sell_shares(ctx, amount, min_price_per_share)
    }

    // ==================== Subscriptions ====================
    
    pub fn create_subscription_tier(
        ctx: Context<CreateSubscriptionTier>,
        name: String,
        description: String,
        price: u64,
        duration_days: u64,
    ) -> Result<()> {
        instructions::subscription::create_subscription_tier(ctx, name, description, price, duration_days)
    }

    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        instructions::subscription::subscribe(ctx)
    }

    pub fn cancel_subscription(ctx: Context<CancelSubscription>) -> Result<()> {
        instructions::subscription::cancel_subscription(ctx)
    }

    // ==================== Groups ====================
    
    pub fn create_group(
        ctx: Context<CreateGroup>,
        name: String,
        description: String,
        privacy: u8,
        entry_requirement: u8,
        entry_price: Option<u64>,
    ) -> Result<()> {
        instructions::group::create_group(ctx, name, description, privacy, entry_requirement, entry_price)
    }

    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
        instructions::group::join_group(ctx)
    }

    pub fn update_member_role(ctx: Context<UpdateMemberRole>, new_role: u8) -> Result<()> {
        instructions::group::update_member_role(ctx, new_role)
    }

    pub fn kick_member(ctx: Context<KickMember>) -> Result<()> {
        instructions::group::kick_member(ctx)
    }

    // ==================== Governance ====================
    
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64, lock_period: u64) -> Result<()> {
        instructions::governance::stake_tokens(ctx, amount, lock_period)
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>) -> Result<()> {
        instructions::governance::unstake_tokens(ctx)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        category: u8,
        execution_delay: i64,
    ) -> Result<()> {
        instructions::governance::create_proposal(ctx, title, description, category, execution_delay)
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_type: u8) -> Result<()> {
        instructions::governance::cast_vote(ctx, vote_type)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        instructions::governance::execute_proposal(ctx)
    }

    // ==================== Marketplace ====================
    
    pub fn mint_username(ctx: Context<MintUsername>, username: String, metadata_uri: String) -> Result<()> {
        instructions::marketplace::mint_username(ctx, username, metadata_uri)
    }

    pub fn list_username(ctx: Context<ListUsername>, price: u64) -> Result<()> {
        instructions::marketplace::list_username(ctx, price)
    }

    pub fn buy_listing(ctx: Context<BuyListing>) -> Result<()> {
        instructions::marketplace::buy_listing(ctx)
    }

    pub fn make_offer(ctx: Context<MakeOffer>, amount: u64) -> Result<()> {
        instructions::marketplace::make_offer(ctx, amount)
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        instructions::marketplace::accept_offer(ctx)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        instructions::marketplace::cancel_offer(ctx)
    }
}
