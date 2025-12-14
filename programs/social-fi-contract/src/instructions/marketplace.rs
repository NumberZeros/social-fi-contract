use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Mint Username ====================

#[derive(Accounts)]
#[instruction(username: String)]
pub struct MintUsername<'info> {
    #[account(
        init,
        payer = owner,
        space = UsernameNFT::LEN,
        seeds = [USERNAME_NFT_SEED, username.as_bytes()],
        bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn mint_username(ctx: Context<MintUsername>, username: String) -> Result<()> {
    require!(
        username.len() <= MAX_USERNAME_LENGTH,
        SocialFiError::UsernameTooLong
    );
    require!(
        username.chars().all(|c| c.is_alphanumeric() || c == '_'),
        SocialFiError::InvalidUsername
    );

    let username_nft = &mut ctx.accounts.username_nft;
    let clock = Clock::get()?;

    username_nft.owner = ctx.accounts.owner.key();
    username_nft.username = username.clone();
    username_nft.verified = false;
    username_nft.minted_at = clock.unix_timestamp;
    username_nft.bump = ctx.bumps.username_nft;

    emit!(UsernameMinted {
        owner: ctx.accounts.owner.key(),
        username,
        nft: ctx.accounts.username_nft.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== List Username ====================

#[derive(Accounts)]
pub struct ListUsername<'info> {
    #[account(
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump,
        constraint = username_nft.owner == seller.key() @ SocialFiError::NotUsernameOwner
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(
        init,
        payer = seller,
        space = Listing::LEN,
        seeds = [LISTING_SEED, username_nft.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn list_username(ctx: Context<ListUsername>, price: u64) -> Result<()> {
    require!(price > 0, SocialFiError::InvalidListingPrice);

    let listing = &mut ctx.accounts.listing;
    let clock = Clock::get()?;
    
    // Determine category based on username length
    let category = if ctx.accounts.username_nft.username.len() <= 3 {
        2 // rare
    } else if ctx.accounts.username_nft.username.len() <= 5 {
        1 // short
    } else {
        3 // custom
    };

    listing.seller = ctx.accounts.seller.key();
    listing.username = ctx.accounts.username_nft.username.clone();
    listing.price = price;
    listing.category = category;
    listing.listed_at = clock.unix_timestamp;
    listing.expires_at = None;
    listing.bump = ctx.bumps.listing;

    emit!(UsernameListed {
        seller: ctx.accounts.seller.key(),
        username: listing.username.clone(),
        price,
        listing: ctx.accounts.listing.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Buy Listing ====================

#[derive(Accounts)]
pub struct BuyListing<'info> {
    #[account(
        mut,
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(
        mut,
        seeds = [LISTING_SEED, username_nft.key().as_ref()],
        bump = listing.bump,
        close = seller
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: Seller address verified through listing
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn buy_listing(ctx: Context<BuyListing>) -> Result<()> {
    // ===== CHECKS =====
    let listing = &ctx.accounts.listing;
    let price = listing.price;
    let seller_key = listing.seller;
    let username = listing.username.clone();
    
    // ===== EFFECTS (Update state BEFORE external calls) =====
    // Transfer NFT ownership
    let username_nft = &mut ctx.accounts.username_nft;
    username_nft.owner = ctx.accounts.buyer.key();

    // ===== INTERACTIONS (External calls LAST) =====
    // Transfer payment to seller
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(cpi_context, price)?;

    let clock = Clock::get()?;
    emit!(UsernameSold {
        seller: seller_key,
        buyer: ctx.accounts.buyer.key(),
        username,
        price,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Make Offer ====================

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(
        seeds = [LISTING_SEED, username_nft.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        init,
        payer = buyer,
        space = Offer::LEN,
        seeds = [OFFER_SEED, listing.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn make_offer(ctx: Context<MakeOffer>, amount: u64) -> Result<()> {
    require!(amount > 0, SocialFiError::InvalidOfferAmount);

    let offer = &mut ctx.accounts.offer;
    let clock = Clock::get()?;
    
    // Offer expires in 7 days
    let expires_at = clock
        .unix_timestamp
        .checked_add(7 * SECONDS_PER_DAY)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    offer.listing = ctx.accounts.listing.key();
    offer.buyer = ctx.accounts.buyer.key();
    offer.amount = amount;
    offer.created_at = clock.unix_timestamp;
    offer.expires_at = expires_at;
    offer.bump = ctx.bumps.offer;

    emit!(OfferMade {
        buyer: ctx.accounts.buyer.key(),
        seller: ctx.accounts.listing.seller,
        listing: ctx.accounts.listing.key(),
        amount,
        expires_at,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Accept Offer ====================

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    #[account(
        mut,
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    #[account(
        mut,
        seeds = [LISTING_SEED, username_nft.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key() @ SocialFiError::NotListingSeller,
        close = seller
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        mut,
        seeds = [OFFER_SEED, listing.key().as_ref(), buyer.key().as_ref()],
        bump = offer.bump,
        close = seller
    )]
    pub offer: Account<'info, Offer>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    // ===== CHECKS =====
    let offer = &ctx.accounts.offer;
    let clock = Clock::get()?;

    require!(
        !offer.is_expired(clock.unix_timestamp),
        SocialFiError::OfferNotFound
    );
    
    let amount = offer.amount;

    // ===== EFFECTS (Update state BEFORE external calls) =====
    // Transfer NFT ownership
    let username_nft = &mut ctx.accounts.username_nft;
    username_nft.owner = ctx.accounts.buyer.key();

    // ===== INTERACTIONS (External calls LAST) =====
    // Transfer payment from buyer to seller
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(cpi_context, amount)?;

    emit!(OfferAccepted {
        seller: ctx.accounts.seller.key(),
        buyer: ctx.accounts.buyer.key(),
        listing: ctx.accounts.listing.key(),
        amount: offer.amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
