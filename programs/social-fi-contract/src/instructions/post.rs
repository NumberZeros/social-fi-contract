use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Create Post (PDA Only) ====================

#[derive(Accounts)]
#[instruction(nonce: String, uri: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = author,
        space = Post::LEN,
        // Use nonce for unique seed per post
        // nonce should be compact string (max 16 chars)
        seeds = [POST_SEED, author.key().as_ref(), nonce.as_bytes()],
        bump
    )]
    pub post: Account<'info, Post>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    #[account(
        seeds = [b"platform_config"],
        bump = platform_config.bump,
        constraint = !platform_config.paused @ crate::errors::SocialFiError::ContractPaused
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_post(ctx: Context<CreatePost>, nonce: String, uri: String) -> Result<()> {
    require!(
        uri.len() <= 200,
        SocialFiError::MetadataUriTooLong
    );
    
    require!(
        nonce.len() <= 16,
        SocialFiError::MetadataUriTooLong  // Reuse error for now
    );

    let post = &mut ctx.accounts.post;
    let clock = Clock::get()?;
    let bump = ctx.bumps.post;

    post.author = ctx.accounts.author.key();
    post.uri = uri;
    post.nonce = nonce;
    post.mint = None;
    post.created_at = clock.unix_timestamp;
    post.bump = bump;

    msg!("Post created: {}", post.key());
    Ok(())
}

// ==================== Mint Post (NFT) ====================

#[derive(Accounts)]
#[instruction(title: String)]
pub struct MintPost<'info> {
    #[account(
        mut,
        has_one = author,
        constraint = post.mint.is_none() @ SocialFiError::PostAlreadyMinted
    )]
    pub post: Account<'info, Post>,
    
    /// SPL Token mint for the NFT
    #[account(
        init,
        payer = author,
        mint::decimals = 0,
        mint::authority = post, // Post PDA is the authority
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
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_post(ctx: Context<MintPost>, title: String, nft_metadata_uri: String) -> Result<()> {
    require!(
        title.len() <= MAX_TITLE_LENGTH,
        SocialFiError::UsernameTooLong // Reusing error for now
    );

    let post_account_info = ctx.accounts.post.to_account_info();
    let mint_account_info = ctx.accounts.mint.to_account_info();
    let token_account_info = ctx.accounts.token_account.to_account_info();
    let metadata_account_info = ctx.accounts.metadata.to_account_info();
    let master_edition_account_info = ctx.accounts.master_edition.to_account_info();
    let author_account_info = ctx.accounts.author.to_account_info();
    let system_program_info = ctx.accounts.system_program.to_account_info();
    let rent_account_info = ctx.accounts.rent.to_account_info();
    let token_program_info = ctx.accounts.token_program.to_account_info();
    let token_metadata_program_info = ctx.accounts.token_metadata_program.to_account_info();

    let post = &mut ctx.accounts.post;
    let clock = Clock::get()?;
    
    // Update post with mint address
    post.mint = Some(ctx.accounts.mint.key());

    // Mint 1 NFT token to author's account
    // Seeds for signing (re-derive seeds using stored nonce)
    let seeds = &[
        POST_SEED,
        post.author.as_ref(),
        post.nonce.as_bytes(),  // CRITICAL: Use stored nonce to match create_post seeds
        &[post.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            token_program_info.clone(),
            anchor_spl::token::MintTo {
                mint: mint_account_info.clone(),
                to: token_account_info,
                authority: post_account_info.clone(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // Create Metaplex metadata account
    let creator = vec![
        mpl_token_metadata::types::Creator {
            address: ctx.accounts.author.key(),
            verified: false, // Must be false - author can verify later by signing
            share: 100,
        },
    ];

    anchor_spl::metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            token_metadata_program_info.clone(),
            anchor_spl::metadata::CreateMetadataAccountsV3 {
                metadata: metadata_account_info.clone(),
                mint: mint_account_info.clone(),
                mint_authority: post_account_info.clone(),
                payer: author_account_info.clone(),
                update_authority: post_account_info.clone(),
                system_program: system_program_info.clone(),
                rent: rent_account_info.clone(),
            },
            signer_seeds,
        ),
        mpl_token_metadata::types::DataV2 {
            name: title,
            symbol: String::from("POST"),
            uri: nft_metadata_uri,
            seller_fee_basis_points: 0, 
            creators: Some(creator),
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    // Create master edition
    anchor_spl::metadata::create_master_edition_v3(
        CpiContext::new_with_signer(
            token_metadata_program_info,
            anchor_spl::metadata::CreateMasterEditionV3 {
                edition: master_edition_account_info,
                mint: mint_account_info,
                update_authority: post_account_info.clone(),
                mint_authority: post_account_info,
                payer: author_account_info,
                metadata: metadata_account_info,
                token_program: token_program_info,
                system_program: system_program_info,
                rent: rent_account_info,
            },
            signer_seeds,
        ),
        Some(0),
    )?;

    emit!(PostMinted {
        author: ctx.accounts.author.key(),
        uri: post.uri.clone(),
        mint: ctx.accounts.mint.key(),
        post: ctx.accounts.post.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
