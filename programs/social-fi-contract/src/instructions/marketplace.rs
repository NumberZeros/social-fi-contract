use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Mint Username ====================

#[derive(Accounts)]
#[instruction(username: String, metadata_uri: String)]
pub struct MintUsername<'info> {
    #[account(
        init,
        payer = owner,
        space = UsernameNFT::LEN,
        seeds = [USERNAME_NFT_SEED, username.as_bytes()],
        bump
    )]
    pub username_nft: Account<'info, UsernameNFT>,
    
    /// SPL Token mint for the NFT
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = username_nft,
        mint::freeze_authority = username_nft,
    )]
    pub mint: Account<'info, Mint>,
    
    /// Owner's associated token account to receive the NFT
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    /// Metaplex metadata account
    /// CHECK: Created via CPI to Metaplex Token Metadata program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    /// Metaplex master edition account
    /// CHECK: Created via CPI to Metaplex Token Metadata program
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_username(ctx: Context<MintUsername>, username: String, metadata_uri: String) -> Result<()> {
    require!(
        username.len() <= MAX_USERNAME_LENGTH,
        SocialFiError::UsernameTooLong
    );
    require!(
        username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.'),
        SocialFiError::InvalidUsername
    );
    require!(
        metadata_uri.len() <= 200,
        SocialFiError::MetadataUriTooLong
    );

    let username_nft = &mut ctx.accounts.username_nft;
    let clock = Clock::get()?;
    let bump = ctx.bumps.username_nft;

    // Store NFT data
    username_nft.owner = ctx.accounts.owner.key();
    username_nft.username = username.clone();
    username_nft.mint = ctx.accounts.mint.key();
    username_nft.metadata_uri = metadata_uri.clone();
    username_nft.verified = false;
    username_nft.minted_at = clock.unix_timestamp;
    username_nft.bump = bump;

    // Mint 1 NFT token to owner's account
    let seeds = &[
        USERNAME_NFT_SEED,
        username.as_bytes(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.username_nft.to_account_info(),
            },
            signer_seeds,
        ),
        1, // Mint exactly 1 NFT
    )?;

    // Create Metaplex metadata account
    let creator = vec![
        mpl_token_metadata::types::Creator {
            address: ctx.accounts.platform_config.key(),
            verified: false, // Platform can verify later if needed
            share: 100, // 100% royalty goes to platform
        },
    ];

    anchor_spl::metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            anchor_spl::metadata::CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.username_nft.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                update_authority: ctx.accounts.username_nft.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        mpl_token_metadata::types::DataV2 {
            name: format!("@{}", username),
            symbol: String::from("USRNM"),
            uri: metadata_uri.clone(),
            seller_fee_basis_points: 500, // 5% royalty
            creators: Some(creator),
            collection: None,
            uses: None,
        },
        true,  // is_mutable
        true,  // update_authority_is_signer
        None,  // collection_details
    )?;

    // Create master edition (makes it a non-fungible token)
    anchor_spl::metadata::create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            anchor_spl::metadata::CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.username_nft.to_account_info(),
                mint_authority: ctx.accounts.username_nft.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        Some(0), // max_supply = 0 means unique 1/1 NFT
    )?;

    emit!(UsernameMinted {
        owner: ctx.accounts.owner.key(),
        username,
        mint: ctx.accounts.mint.key(),
        nft: ctx.accounts.username_nft.key(),
        metadata_uri,
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
    
    /// Seller's token account - must hold the NFT to list
    #[account(
        constraint = seller_token_account.mint == username_nft.mint @ SocialFiError::InvalidListingPrice,
        constraint = seller_token_account.owner == seller.key() @ SocialFiError::NotUsernameOwner,
        constraint = seller_token_account.amount == 1 @ SocialFiError::InsufficientShares,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
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
    pub token_program: Program<'info, Token>,
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
    
    /// SPL Token mint account for the NFT
    pub mint: Account<'info, Mint>,
    
    /// Seller's token account holding the NFT
    #[account(
        mut,
        constraint = seller_token_account.mint == mint.key() @ SocialFiError::InvalidListingPrice,
        constraint = seller_token_account.owner == listing.seller @ SocialFiError::NotUsernameOwner,
        constraint = seller_token_account.amount == 1 @ SocialFiError::InsufficientShares,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    /// Buyer's token account to receive the NFT
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
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
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn buy_listing(ctx: Context<BuyListing>) -> Result<()> {
    // ===== CHECKS =====
    let listing = &ctx.accounts.listing;
    let price = listing.price;
    let seller_key = listing.seller;
    let username = listing.username.clone();
    
    let clock = Clock::get()?;
    
    // Check listing hasn't expired
    if let Some(expires_at) = listing.expires_at {
        require!(
            clock.unix_timestamp < expires_at,
            SocialFiError::ListingNotFound
        );
    }
    
    // ===== EFFECTS (Update state BEFORE external calls) =====
    // Transfer NFT ownership in PDA
    let username_nft = &mut ctx.accounts.username_nft;
    username_nft.owner = ctx.accounts.buyer.key();

    // ===== INTERACTIONS (External calls LAST) =====
    // Transfer SPL token from seller to buyer
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        1, // Transfer 1 NFT
    )?;
    
    // Transfer SOL payment to seller
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(cpi_context, price)?;

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
    require!(amount >= MIN_OFFER_AMOUNT, SocialFiError::InvalidOfferAmount);

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
    
    /// SPL Token mint account for the NFT
    pub mint: Account<'info, Mint>,
    
    /// Seller's token account holding the NFT
    #[account(
        mut,
        constraint = seller_token_account.mint == mint.key() @ SocialFiError::InvalidListingPrice,
        constraint = seller_token_account.owner == seller.key() @ SocialFiError::NotUsernameOwner,
        constraint = seller_token_account.amount == 1 @ SocialFiError::InsufficientShares,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    /// Buyer's token account to receive the NFT
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    /// CHECK: Buyer verified through offer.buyer - no signature needed (funds escrowed)
    #[account(mut)]
    pub buyer: AccountInfo<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    // ===== CHECKS =====
    let offer = &ctx.accounts.offer;
    let clock = Clock::get()?;

    require!(
        !offer.is_expired(clock.unix_timestamp),
        SocialFiError::OfferNotFound
    );

    // ===== EFFECTS (Update state BEFORE external calls) =====
    // Transfer NFT ownership in PDA
    let username_nft = &mut ctx.accounts.username_nft;
    username_nft.owner = ctx.accounts.buyer.key();

    // ===== INTERACTIONS (External calls LAST) =====
    // Transfer SPL token from seller to buyer
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        1, // Transfer 1 NFT
    )?;
    
    // ===== ESCROW: Withdraw SOL from offer PDA to seller =====
    let offer_lamports = ctx.accounts.offer.to_account_info().lamports();
    let rent_exempt = Rent::get()?.minimum_balance(ctx.accounts.offer.to_account_info().data_len());
    let payment_amount = offer_lamports.checked_sub(rent_exempt).unwrap_or(0);
    
    **ctx.accounts.offer.to_account_info().try_borrow_mut_lamports()? -= payment_amount;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += payment_amount;

    emit!(OfferAccepted {
        seller: ctx.accounts.seller.key(),
        buyer: ctx.accounts.buyer.key(),
        listing: ctx.accounts.listing.key(),
        amount: offer.amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Cancel Offer ====================

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(
        mut,
        seeds = [OFFER_SEED, offer.listing.as_ref(), buyer.key().as_ref()],
        bump = offer.bump,
        constraint = offer.buyer == buyer.key() @ SocialFiError::Unauthorized,
        close = buyer  // Rent refund goes to buyer
    )]
    pub offer: Account<'info, Offer>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
    let offer = &ctx.accounts.offer;
    let clock = Clock::get()?;
    
    // Allow cancellation even if expired (buyer gets refund either way)
    let amount = offer.amount;
    
    // ===== ESCROW: Withdraw lamports from offer account =====
    // Offer PDA is an Account (has data), not pure SystemAccount
    // Must withdraw lamports directly, not via transfer()
    let offer_lamports = ctx.accounts.offer.to_account_info().lamports();
    let rent_exempt = Rent::get()?.minimum_balance(ctx.accounts.offer.to_account_info().data_len());
    let refund_amount = offer_lamports.checked_sub(rent_exempt).unwrap_or(0);
    
    **ctx.accounts.offer.to_account_info().try_borrow_mut_lamports()? -= refund_amount;
    **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += refund_amount;
    
    emit!(OfferCancelled {
        buyer: ctx.accounts.buyer.key(),
        listing: offer.listing,
        amount,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}

// ==================== Cancel Listing ====================

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(
        mut,
        seeds = [USERNAME_NFT_SEED, username_nft.username.as_bytes()],
        bump = username_nft.bump,
        constraint = username_nft.owner == seller.key() @ SocialFiError::NotUsernameOwner
    )]
    pub username_nft: Account<'info, UsernameNFT>,

    #[account(
        mut,
        seeds = [LISTING_SEED, username_nft.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key() @ SocialFiError::NotUsernameOwner,
        close = seller
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
    let clock = Clock::get()?;

    emit!(ListingCancelled {
        seller: ctx.accounts.seller.key(),
        username: ctx.accounts.listing.username.clone(),
        listing: ctx.accounts.listing.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
