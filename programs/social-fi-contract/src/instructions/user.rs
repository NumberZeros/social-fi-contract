use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Initialize User ====================

#[derive(Accounts)]
#[instruction(username: String)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = user,
        space = UserProfile::LEN,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_user(ctx: Context<InitializeUser>, username: String) -> Result<()> {
    require!(
        username.len() <= MAX_USERNAME_LENGTH,
        SocialFiError::UsernameTooLong
    );
    
    require!(
        username.chars().all(|c| c.is_alphanumeric() || c == '_'),
        SocialFiError::InvalidUsername
    );

    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    user_profile.owner = ctx.accounts.user.key();
    user_profile.username = username.clone();
    user_profile.total_tips_sent = 0;
    user_profile.total_tips_received = 0;
    user_profile.posts_count = 0;
    user_profile.followers_count = 0;
    user_profile.following_count = 0;
    user_profile.referral_code = generate_referral_code(&ctx.accounts.user.key());
    user_profile.referred_by = None;
    user_profile.referrals_count = 0;
    user_profile.created_at = clock.unix_timestamp;
    user_profile.bump = ctx.bumps.user_profile;

    emit!(UserInitialized {
        user: ctx.accounts.user.key(),
        username,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

fn generate_referral_code(pubkey: &Pubkey) -> String {
    let bytes = pubkey.to_bytes();
    bs58::encode(&bytes[..6]).into_string()
}

// ==================== Send Tip ====================

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, sender.key().as_ref()],
        bump = sender_profile.bump
    )]
    pub sender_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, recipient.key().as_ref()],
        bump = recipient_profile.bump
    )]
    pub recipient_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub sender: Signer<'info>,
    
    /// CHECK: Recipient address verified through PDA
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn send_tip(ctx: Context<SendTip>, amount: u64) -> Result<()> {
    // ===== CHECKS =====
    require!(amount > 0, SocialFiError::InvalidAmount);
    require!(
        ctx.accounts.sender.key() != ctx.accounts.recipient.key(),
        SocialFiError::CannotTipSelf
    );

    // ===== EFFECTS (Update state BEFORE external calls) =====
    let sender_profile = &mut ctx.accounts.sender_profile;
    let recipient_profile = &mut ctx.accounts.recipient_profile;
    
    sender_profile.total_tips_sent = sender_profile
        .total_tips_sent
        .checked_add(amount)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    recipient_profile.total_tips_received = recipient_profile
        .total_tips_received
        .checked_add(amount)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    // ===== INTERACTIONS (External calls LAST) =====
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        },
    );
    transfer(cpi_context, amount)?;

    let clock = Clock::get()?;
    emit!(TipSent {
        sender: ctx.accounts.sender.key(),
        recipient: ctx.accounts.recipient.key(),
        amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
