use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Initialize Creator Pool ====================

#[derive(Accounts)]
pub struct InitializeCreatorPool<'info> {
    #[account(
        init,
        payer = creator,
        space = CreatorPool::LEN,
        seeds = [CREATOR_POOL_SEED, creator.key().as_ref()],
        bump
    )]
    pub creator_pool: Account<'info, CreatorPool>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_creator_pool(ctx: Context<InitializeCreatorPool>) -> Result<()> {
    let creator_pool = &mut ctx.accounts.creator_pool;
    let clock = Clock::get()?;

    creator_pool.creator = ctx.accounts.creator.key();
    creator_pool.supply = 0;
    creator_pool.holders_count = 0;
    creator_pool.base_price = BASE_PRICE;
    creator_pool.total_volume = 0;
    creator_pool.created_at = clock.unix_timestamp;
    creator_pool.bump = ctx.bumps.creator_pool;

    Ok(())
}

// ==================== Buy Shares ====================

#[derive(Accounts)]
pub struct BuyShares<'info> {
    #[account(
        mut,
        seeds = [CREATOR_POOL_SEED, creator.key().as_ref()],
        bump = creator_pool.bump
    )]
    pub creator_pool: Account<'info, CreatorPool>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        space = ShareHolding::LEN,
        seeds = [SHARE_HOLDING_SEED, buyer.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub share_holding: Account<'info, ShareHolding>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: Creator address verified through PDA
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn buy_shares(ctx: Context<BuyShares>, amount: u64) -> Result<()> {
    require!(amount > 0, SocialFiError::InvalidAmount);

    let creator_pool = &mut ctx.accounts.creator_pool;
    let share_holding = &mut ctx.accounts.share_holding;
    
    // Calculate total cost
    let total_cost = creator_pool.calculate_buy_cost(amount)?;
    
    // Transfer payment to creator
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.creator.to_account_info(),
        },
    );
    transfer(cpi_context, total_cost)?;

    // Update creator pool
    let is_new_holder = share_holding.amount == 0;
    
    creator_pool.supply = creator_pool
        .supply
        .checked_add(amount)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    creator_pool.total_volume = creator_pool
        .total_volume
        .checked_add(total_cost)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    if is_new_holder {
        creator_pool.holders_count = creator_pool
            .holders_count
            .checked_add(1)
            .ok_or(SocialFiError::ArithmeticOverflow)?;
    }

    // Update share holding
    let new_amount = share_holding
        .amount
        .checked_add(amount)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    // Calculate new average price
    let total_value = share_holding
        .amount
        .checked_mul(share_holding.average_price)
        .ok_or(SocialFiError::ArithmeticOverflow)?
        .checked_add(total_cost)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    share_holding.average_price = total_value
        .checked_div(new_amount)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;
    
    share_holding.amount = new_amount;
    share_holding.holder = ctx.accounts.buyer.key();
    share_holding.creator = ctx.accounts.creator.key();
    
    if share_holding.created_at == 0 {
        let clock = Clock::get()?;
        share_holding.created_at = clock.unix_timestamp;
        share_holding.bump = ctx.bumps.share_holding;
    }

    // Calculate average price for event
    let avg_price = total_cost
        .checked_div(amount)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;

    let clock = Clock::get()?;
    emit!(SharesPurchased {
        buyer: ctx.accounts.buyer.key(),
        creator: ctx.accounts.creator.key(),
        amount,
        price: avg_price,
        total_cost,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Sell Shares ====================

#[derive(Accounts)]
pub struct SellShares<'info> {
    #[account(
        mut,
        seeds = [CREATOR_POOL_SEED, creator.key().as_ref()],
        bump = creator_pool.bump
    )]
    pub creator_pool: Account<'info, CreatorPool>,
    
    #[account(
        mut,
        seeds = [SHARE_HOLDING_SEED, seller.key().as_ref(), creator.key().as_ref()],
        bump = share_holding.bump
    )]
    pub share_holding: Account<'info, ShareHolding>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    /// CHECK: Creator address verified through PDA
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn sell_shares(ctx: Context<SellShares>, amount: u64) -> Result<()> {
    require!(amount > 0, SocialFiError::InvalidAmount);
    
    let share_holding = &ctx.accounts.share_holding;
    require!(
        share_holding.amount >= amount,
        SocialFiError::InsufficientShares
    );

    let creator_pool = &mut ctx.accounts.creator_pool;
    
    // Calculate sell return (after 10% fee)
    let total_return = creator_pool.calculate_sell_return(amount)?;
    let fee = total_return
        .checked_mul(SELL_FEE_BPS)
        .ok_or(SocialFiError::ArithmeticOverflow)?
        .checked_div(BPS_DENOMINATOR)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;
    
    let seller_receives = total_return
        .checked_sub(fee)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;
    
    // Transfer from creator to seller (creator received funds on buy)
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.creator.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(cpi_context, seller_receives)?;

    // Update creator pool
    creator_pool.supply = creator_pool
        .supply
        .checked_sub(amount)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;
    
    creator_pool.total_volume = creator_pool
        .total_volume
        .checked_add(total_return)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    // Update share holding
    let share_holding = &mut ctx.accounts.share_holding;
    share_holding.amount = share_holding
        .amount
        .checked_sub(amount)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;
    
    // If holder sold all shares, decrement holders count
    if share_holding.amount == 0 {
        creator_pool.holders_count = creator_pool
            .holders_count
            .checked_sub(1)
            .ok_or(SocialFiError::ArithmeticUnderflow)?;
    }

    // Calculate average price for event
    let avg_price = total_return
        .checked_div(amount)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;

    let clock = Clock::get()?;
    emit!(SharesSold {
        seller: ctx.accounts.seller.key(),
        creator: ctx.accounts.creator.key(),
        amount,
        price: avg_price,
        total_received: seller_receives,
        fee,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
