use anchor_lang::prelude::*;
use crate::state::*;

// ==================== Initialize Platform ====================

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        payer = admin,
        space = PlatformConfig::LEN,
        seeds = [b"platform_config"],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_platform(
    ctx: Context<InitializePlatform>,
    fee_collector: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.platform_config;
    
    config.admin = ctx.accounts.admin.key();
    config.fee_collector = fee_collector;
    config.paused = false;
    config.min_liquidity_bps = 1000; // 10% default
    config.bump = ctx.bumps.platform_config;
    
    Ok(())
}

// ==================== Update Platform ====================

#[derive(Accounts)]
pub struct UpdatePlatform<'info> {
    #[account(
        mut,
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = platform_config.admin == admin.key() @ crate::errors::SocialFiError::Unauthorized
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub admin: Signer<'info>,
}

pub fn pause_platform(ctx: Context<UpdatePlatform>) -> Result<()> {
    ctx.accounts.platform_config.paused = true;
    Ok(())
}

pub fn unpause_platform(ctx: Context<UpdatePlatform>) -> Result<()> {
    ctx.accounts.platform_config.paused = false;
    Ok(())
}

pub fn update_admin(ctx: Context<UpdatePlatform>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.platform_config.admin = new_admin;
    Ok(())
}

pub fn update_fee_collector(ctx: Context<UpdatePlatform>, new_fee_collector: Pubkey) -> Result<()> {
    ctx.accounts.platform_config.fee_collector = new_fee_collector;
    Ok(())
}

pub fn update_min_liquidity(ctx: Context<UpdatePlatform>, new_min_liquidity_bps: u64) -> Result<()> {
    require!(
        new_min_liquidity_bps <= 5000, // Max 50%
        crate::errors::SocialFiError::InvalidAmount
    );
    ctx.accounts.platform_config.min_liquidity_bps = new_min_liquidity_bps;
    Ok(())
}
