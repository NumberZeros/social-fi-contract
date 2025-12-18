use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::constants::*;

// ==================== Follow User ====================

#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(
        init,
        payer = follower,
        space = Follow::LEN,
        seeds = [FOLLOW_SEED, follower.key().as_ref(), following.key().as_ref()],
        bump
    )]
    pub follow: Account<'info, Follow>,
    
    #[account(mut)]
    pub follower: Signer<'info>,
    
    /// CHECK: The user being followed (just their pubkey)
    pub following: UncheckedAccount<'info>,
    
    /// Follower's profile (optional - to update count)
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, follower.key().as_ref()],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, UserProfile>,
    
    /// Following's profile (optional - to update count)
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, following.key().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, UserProfile>,
    
    pub system_program: Program<'info, System>,
}

pub fn follow_user(ctx: Context<FollowUser>) -> Result<()> {
    let follow = &mut ctx.accounts.follow;
    let clock = Clock::get()?;

    follow.follower = ctx.accounts.follower.key();
    follow.following = ctx.accounts.following.key();
    follow.created_at = clock.unix_timestamp;
    follow.bump = ctx.bumps.follow;

    // Update profile counts
    ctx.accounts.follower_profile.following_count = ctx.accounts.follower_profile.following_count.saturating_add(1);
    ctx.accounts.following_profile.followers_count = ctx.accounts.following_profile.followers_count.saturating_add(1);

    msg!("User {} followed {}", ctx.accounts.follower.key(), ctx.accounts.following.key());
    Ok(())
}

// ==================== Unfollow User ====================

#[derive(Accounts)]
pub struct UnfollowUser<'info> {
    #[account(
        mut,
        close = follower,
        seeds = [FOLLOW_SEED, follower.key().as_ref(), following.key().as_ref()],
        bump = follow.bump,
        has_one = follower,
    )]
    pub follow: Account<'info, Follow>,
    
    #[account(mut)]
    pub follower: Signer<'info>,
    
    /// CHECK: The user being unfollowed
    pub following: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, follower.key().as_ref()],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, following.key().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, UserProfile>,
}

pub fn unfollow_user(ctx: Context<UnfollowUser>) -> Result<()> {
    // Update profile counts
    ctx.accounts.follower_profile.following_count = ctx.accounts.follower_profile.following_count.saturating_sub(1);
    ctx.accounts.following_profile.followers_count = ctx.accounts.following_profile.followers_count.saturating_sub(1);

    msg!("User {} unfollowed {}", ctx.accounts.follower.key(), ctx.accounts.following.key());
    Ok(())
}

// ==================== Like Post ====================

#[derive(Accounts)]
pub struct LikePost<'info> {
    #[account(
        init,
        payer = user,
        space = Like::LEN,
        seeds = [LIKE_SEED, user.key().as_ref(), post.key().as_ref()],
        bump
    )]
    pub like: Account<'info, Like>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The post being liked
    #[account(mut)]
    pub post: Account<'info, Post>,
    
    pub system_program: Program<'info, System>,
}

pub fn like_post(ctx: Context<LikePost>) -> Result<()> {
    let like = &mut ctx.accounts.like;
    let clock = Clock::get()?;

    like.user = ctx.accounts.user.key();
    like.post = ctx.accounts.post.key();
    like.created_at = clock.unix_timestamp;
    like.bump = ctx.bumps.like;

    msg!("User {} liked post {}", ctx.accounts.user.key(), ctx.accounts.post.key());
    Ok(())
}

// ==================== Unlike Post ====================

#[derive(Accounts)]
pub struct UnlikePost<'info> {
    #[account(
        mut,
        close = user,
        seeds = [LIKE_SEED, user.key().as_ref(), post.key().as_ref()],
        bump = like.bump,
        has_one = user,
        has_one = post,
    )]
    pub like: Account<'info, Like>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub post: Account<'info, Post>,
}

pub fn unlike_post(ctx: Context<UnlikePost>) -> Result<()> {
    msg!("User {} unliked post {}", ctx.accounts.user.key(), ctx.accounts.post.key());
    Ok(())
}

// ==================== Repost ====================

#[derive(Accounts)]
pub struct CreateRepost<'info> {
    #[account(
        init,
        payer = user,
        space = Repost::LEN,
        seeds = [REPOST_SEED, user.key().as_ref(), original_post.key().as_ref()],
        bump
    )]
    pub repost: Account<'info, Repost>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The post being reposted
    pub original_post: Account<'info, Post>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_repost(ctx: Context<CreateRepost>) -> Result<()> {
    let repost = &mut ctx.accounts.repost;
    let clock = Clock::get()?;

    repost.user = ctx.accounts.user.key();
    repost.original_post = ctx.accounts.original_post.key();
    repost.created_at = clock.unix_timestamp;
    repost.bump = ctx.bumps.repost;

    msg!("User {} reposted post {}", ctx.accounts.user.key(), ctx.accounts.original_post.key());
    Ok(())
}

// ==================== Comment ====================

#[derive(Accounts)]
#[instruction(nonce: u64, content: String)]
pub struct CreateComment<'info> {
    #[account(
        init,
        payer = author,
        space = Comment::LEN,
        seeds = [COMMENT_SEED, post.key().as_ref(), author.key().as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub comment: Account<'info, Comment>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    /// The post being commented on
    pub post: Account<'info, Post>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_comment(ctx: Context<CreateComment>, _nonce: u64, content: String) -> Result<()> {
    require!(
        content.len() <= 280,
        SocialFiError::CommentTooLong
    );
    require!(
        !content.trim().is_empty(),
        SocialFiError::EmptyContent
    );

    let comment = &mut ctx.accounts.comment;
    let clock = Clock::get()?;

    comment.author = ctx.accounts.author.key();
    comment.post = ctx.accounts.post.key();
    comment.content = content;
    comment.created_at = clock.unix_timestamp;
    comment.bump = ctx.bumps.comment;

    msg!("User {} commented on post {}", ctx.accounts.author.key(), ctx.accounts.post.key());
    Ok(())
}
