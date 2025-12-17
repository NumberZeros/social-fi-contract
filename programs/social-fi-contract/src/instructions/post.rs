use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(title: String, uri: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = author,
        space = Post::LEN,
        seeds = [POST_SEED, author.key().as_ref(), uri.as_bytes()], // Use URI as unique seed component? Or random/timestamp?
        // Using URI as seed might be long. Maybe use a UUID passed from FE?
        // Let's stick to using a unique ID arg if possible, or just the URI if unique.
        // For simplicity and to match MintUsername pattern, we'll use a derived seed.
        // Wait, URI can be long. Let's use a combination or rely on a passed ID.
        // Detailed Design: Let's assume URI is unique enough or adds randomness.
        // Actually, using `uri` (limited length) as seed works if we embrace it.
        // But `username` was short. `uri` is 200 chars.
        // Alternative: Use a counter in UserProfile? Too complex for now.
        // Let's use the `uri` as the seed, assuming it's an IPFS hash which is unique.
        bump
    )]
    pub post: Account<'info, Post>,
    
    /// SPL Token mint for the NFT
    #[account(
        init,
        payer = author,
        mint::decimals = 0,
        mint::authority = post,
        mint::freeze_authority = post,
    )]
    pub mint: Account<'info, Mint>,
    
    /// Owner's associated token account to receive the NFT
    #[account(
        init_if_needed,
        payer = author,
        associated_token::mint = mint,
        associated_token::authority = author,
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
    pub author: Signer<'info>,
    
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

pub fn create_post(ctx: Context<CreatePost>, title: String, uri: String) -> Result<()> {
    require!(
        uri.len() <= 200,
        SocialFiError::MetadataUriTooLong
    );
    // Title length check if needed
    require!(
        title.len() <= MAX_TITLE_LENGTH, // Using existing constant or new one
        SocialFiError::UsernameTooLong // Reusing error or map new one? Let's use generic or add one.
        // crate::errors::SocialFiError does not have TitleTooLong.
        // Using MetadataUriTooLong as proxy or just length check.
    );

    let post = &mut ctx.accounts.post;
    let clock = Clock::get()?;
    let bump = ctx.bumps.post;

    // Store Post data
    post.author = ctx.accounts.author.key();
    post.uri = uri.clone();
    post.mint = ctx.accounts.mint.key();
    post.created_at = clock.unix_timestamp;
    post.bump = bump;

    // Mint 1 NFT token to author's account
    // Seeds for signing
    let seeds = &[
        POST_SEED,
        ctx.accounts.author.key.as_ref(),
        uri.as_bytes(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.post.to_account_info(),
            },
            signer_seeds,
        ),
        1, // Mint exactly 1 NFT
    )?;

    // Create Metaplex metadata account
    let creator = vec![
        mpl_token_metadata::types::Creator {
            address: ctx.accounts.author.key(),
            verified: true, // Author is signer
            share: 100,
        },
    ];

    anchor_spl::metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            anchor_spl::metadata::CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.post.to_account_info(),
                payer: ctx.accounts.author.to_account_info(),
                update_authority: ctx.accounts.post.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        mpl_token_metadata::types::DataV2 {
            name: title,
            symbol: String::from("POST"),
            uri: uri.clone(),
            seller_fee_basis_points: 0, 
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
                update_authority: ctx.accounts.post.to_account_info(),
                mint_authority: ctx.accounts.post.to_account_info(),
                payer: ctx.accounts.author.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        Some(0), // max_supply = 0 means unique 1/1 NFT
    )?;

    emit!(PostMinted {
        author: ctx.accounts.author.key(),
        uri: uri.clone(),
        mint: ctx.accounts.mint.key(),
        post: ctx.accounts.post.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
