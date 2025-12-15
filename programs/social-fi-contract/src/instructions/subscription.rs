use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Create Subscription Tier ====================

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateSubscriptionTier<'info> {
    #[account(
        init,
        payer = creator,
        space = SubscriptionTier::LEN,
        seeds = [
            SUBSCRIPTION_TIER_SEED,
            creator.key().as_ref(),
            &get_next_tier_id().to_le_bytes()
        ],
        bump
    )]
    pub subscription_tier: Account<'info, SubscriptionTier>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_subscription_tier(
    ctx: Context<CreateSubscriptionTier>,
    name: String,
    description: String,
    price: u64,
    duration_days: u64,
) -> Result<()> {
    require!(
        name.len() <= MAX_NAME_LENGTH,
        SocialFiError::GroupNameTooLong
    );
    require!(
        description.len() <= MAX_DESCRIPTION_LENGTH,
        SocialFiError::ProposalDescriptionTooLong
    );
    require!(price > 0, SocialFiError::InvalidAmount);
    require!(duration_days > 0, SocialFiError::InvalidAmount);

    let subscription_tier = &mut ctx.accounts.subscription_tier;
    let clock = Clock::get()?;
    let tier_id = get_next_tier_id();

    subscription_tier.creator = ctx.accounts.creator.key();
    subscription_tier.tier_id = tier_id;
    subscription_tier.name = name.clone();
    subscription_tier.description = description;
    subscription_tier.price = price;
    subscription_tier.duration_days = duration_days;
    subscription_tier.subscriber_count = 0;
    subscription_tier.created_at = clock.unix_timestamp;
    subscription_tier.bump = ctx.bumps.subscription_tier;

    emit!(SubscriptionTierCreated {
        creator: ctx.accounts.creator.key(),
        tier_id,
        name,
        price,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

fn get_next_tier_id() -> u64 {
    // In production, this should be a counter PDA
    // For now, using timestamp as unique ID
    // Using a fixed value for IDL generation
    1
}

// ==================== Subscribe ====================

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        seeds = [
            SUBSCRIPTION_TIER_SEED,
            creator.key().as_ref(),
            &subscription_tier.tier_id.to_le_bytes()
        ],
        bump = subscription_tier.bump
    )]
    pub subscription_tier: Account<'info, SubscriptionTier>,
    
    #[account(
        init,
        payer = subscriber,
        space = Subscription::LEN,
        seeds = [
            SUBSCRIPTION_SEED,
            subscriber.key().as_ref(),
            creator.key().as_ref(),
            &subscription_tier.tier_id.to_le_bytes()
        ],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
    
    /// CHECK: Creator address verified through subscription tier
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
    let subscription_tier = &ctx.accounts.subscription_tier;
    let subscription = &mut ctx.accounts.subscription;
    let clock = Clock::get()?;

    // ===== CHECKS =====
    let price = subscription_tier.price;
    let tier_id = subscription_tier.tier_id;

    // Calculate end date
    let duration_seconds = subscription_tier
        .duration_days
        .checked_mul(SECONDS_PER_DAY as u64)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    let end_date = clock
        .unix_timestamp
        .checked_add(duration_seconds as i64)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    // ===== EFFECTS (Update state BEFORE external calls) =====
    // Initialize subscription
    subscription.subscriber = ctx.accounts.subscriber.key();
    subscription.creator = ctx.accounts.creator.key();
    subscription.tier_id = tier_id;
    subscription.start_date = clock.unix_timestamp;
    subscription.end_date = end_date;
    subscription.status = 0; // active
    subscription.auto_renew = false;
    subscription.created_at = clock.unix_timestamp;
    subscription.bump = ctx.bumps.subscription;

    // Update subscriber count
    let subscription_tier = &mut ctx.accounts.subscription_tier;
    subscription_tier.subscriber_count = subscription_tier
        .subscriber_count
        .checked_add(1)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    // ===== INTERACTIONS (External calls LAST) =====
    // Transfer payment to creator
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.subscriber.to_account_info(),
            to: ctx.accounts.creator.to_account_info(),
        },
    );
    transfer(cpi_context, price)?;

    emit!(UserSubscribed {
        subscriber: ctx.accounts.subscriber.key(),
        creator: ctx.accounts.creator.key(),
        tier_id: subscription_tier.tier_id,
        start_date: clock.unix_timestamp,
        end_date,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Cancel Subscription ====================

#[derive(Accounts)]
pub struct CancelSubscription<'info> {
    #[account(
        mut,
        seeds = [
            SUBSCRIPTION_SEED,
            subscriber.key().as_ref(),
            subscription.creator.as_ref(),
            &subscription.tier_id.to_le_bytes()
        ],
        bump = subscription.bump,
        constraint = subscription.subscriber == subscriber.key()
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
}

pub fn cancel_subscription(ctx: Context<CancelSubscription>) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;
    let clock = Clock::get()?;

    require!(
        subscription.is_active(clock.unix_timestamp),
        SocialFiError::SubscriptionInactive
    );

    subscription.status = 2; // cancelled
    subscription.auto_renew = false;

    emit!(SubscriptionCancelled {
        subscriber: ctx.accounts.subscriber.key(),
        creator: subscription.creator,
        tier_id: subscription.tier_id,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
