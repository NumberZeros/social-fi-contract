use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Create Group ====================

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        payer = creator,
        space = Group::LEN,
        seeds = [GROUP_SEED, creator.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub group: Account<'info, Group>,
    
    #[account(
        init,
        payer = creator,
        space = GroupMember::LEN,
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub group_member: Account<'info, GroupMember>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_group(
    ctx: Context<CreateGroup>,
    name: String,
    description: String,
    privacy: u8,
    entry_requirement: u8,
    entry_price: Option<u64>,
) -> Result<()> {
    require!(
        name.len() <= MAX_NAME_LENGTH,
        SocialFiError::GroupNameTooLong
    );
    require!(
        description.len() <= MAX_DESCRIPTION_LENGTH,
        SocialFiError::ProposalDescriptionTooLong
    );
    require!(privacy <= 2, SocialFiError::InvalidGroupPrivacy);
    require!(entry_requirement <= 3, SocialFiError::InvalidEntryRequirement);

    let clock = Clock::get()?;
    let group_key = ctx.accounts.group.key();
    
    let group = &mut ctx.accounts.group;
    let group_member = &mut ctx.accounts.group_member;

    group.id = group_key;
    group.name = name.clone();
    group.description = description;
    group.creator = ctx.accounts.creator.key();
    group.privacy = privacy;
    group.entry_requirement = entry_requirement;
    group.entry_price = entry_price;
    group.token_mint = None;
    group.nft_collection = None;
    group.member_count = 1;
    group.post_count = 0;
    group.created_at = clock.unix_timestamp;
    group.bump = ctx.bumps.group;

    // Initialize creator as owner
    group_member.group = ctx.accounts.group.key();
    group_member.wallet = ctx.accounts.creator.key();
    group_member.role = 0; // owner
    group_member.joined_at = clock.unix_timestamp;
    group_member.banned = false;
    group_member.bump = ctx.bumps.group_member;

    emit!(GroupCreated {
        group: ctx.accounts.group.key(),
        creator: ctx.accounts.creator.key(),
        name,
        privacy,
        timestamp: clock.unix_timestamp,
    });

    emit!(MemberJoined {
        group: ctx.accounts.group.key(),
        member: ctx.accounts.creator.key(),
        role: 0,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Join Group ====================

#[derive(Accounts)]
pub struct JoinGroup<'info> {
    #[account(
        mut,
        seeds = [GROUP_SEED, group.creator.as_ref(), group.name.as_bytes()],
        bump = group.bump
    )]
    pub group: Account<'info, Group>,
    
    #[account(
        init,
        payer = member,
        space = GroupMember::LEN,
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), member.key().as_ref()],
        bump
    )]
    pub group_member: Account<'info, GroupMember>,
    
    #[account(mut)]
    pub member: Signer<'info>,
    
    /// CHECK: Group creator address for payment
    #[account(mut)]
    pub group_creator: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
    let group = &ctx.accounts.group;
    let clock = Clock::get()?;

    // Handle entry requirement
    match group.entry_requirement {
        1 => {
            // Pay SOL
            if let Some(price) = group.entry_price {
                let cpi_context = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.member.to_account_info(),
                        to: ctx.accounts.group_creator.to_account_info(),
                    },
                );
                transfer(cpi_context, price)?;
            }
        }
        2 | 3 => {
            // Token/NFT verification would go here
            // For now, we'll skip actual token checking
            // In production, use token account verification
        }
        _ => {} // free entry
    }

    // Initialize member
    let group_member = &mut ctx.accounts.group_member;
    group_member.group = ctx.accounts.group.key();
    group_member.wallet = ctx.accounts.member.key();
    group_member.role = 3; // member
    group_member.joined_at = clock.unix_timestamp;
    group_member.banned = false;
    group_member.bump = ctx.bumps.group_member;

    // Update member count
    let group = &mut ctx.accounts.group;
    group.member_count = group
        .member_count
        .checked_add(1)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    emit!(MemberJoined {
        group: ctx.accounts.group.key(),
        member: ctx.accounts.member.key(),
        role: 3,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Update Member Role ====================

#[derive(Accounts)]
pub struct UpdateMemberRole<'info> {
    #[account(
        seeds = [GROUP_SEED, group.creator.as_ref(), group.name.as_bytes()],
        bump = group.bump
    )]
    pub group: Account<'info, Group>,
    
    #[account(
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), admin.key().as_ref()],
        bump = admin_member.bump,
        constraint = admin_member.can_manage_members() @ SocialFiError::InsufficientPermissions
    )]
    pub admin_member: Account<'info, GroupMember>,
    
    #[account(
        mut,
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), target_member.wallet.as_ref()],
        bump = target_member.bump
    )]
    pub target_member: Account<'info, GroupMember>,
    
    pub admin: Signer<'info>,
}

pub fn update_member_role(ctx: Context<UpdateMemberRole>, new_role: u8) -> Result<()> {
    require!(new_role >= 1 && new_role <= 3, SocialFiError::InsufficientPermissions);
    require!(
        ctx.accounts.admin.key() != ctx.accounts.target_member.wallet,
        SocialFiError::CannotActOnSelf
    );

    let target_member = &mut ctx.accounts.target_member;
    let old_role = target_member.role;
    target_member.role = new_role;

    let clock = Clock::get()?;
    emit!(MemberRoleUpdated {
        group: ctx.accounts.group.key(),
        member: target_member.wallet,
        old_role,
        new_role,
        updated_by: ctx.accounts.admin.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Kick Member ====================

#[derive(Accounts)]
pub struct KickMember<'info> {
    #[account(
        mut,
        seeds = [GROUP_SEED, group.creator.as_ref(), group.name.as_bytes()],
        bump = group.bump
    )]
    pub group: Account<'info, Group>,
    
    #[account(
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), admin.key().as_ref()],
        bump = admin_member.bump,
        constraint = admin_member.can_manage_members() @ SocialFiError::InsufficientPermissions
    )]
    pub admin_member: Account<'info, GroupMember>,
    
    #[account(
        mut,
        seeds = [GROUP_MEMBER_SEED, group.key().as_ref(), target_member.wallet.as_ref()],
        bump = target_member.bump,
        close = admin
    )]
    pub target_member: Account<'info, GroupMember>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn kick_member(ctx: Context<KickMember>) -> Result<()> {
    require!(
        ctx.accounts.admin.key() != ctx.accounts.target_member.wallet,
        SocialFiError::CannotActOnSelf
    );

    // Update member count
    let group = &mut ctx.accounts.group;
    group.member_count = group
        .member_count
        .checked_sub(1)
        .ok_or(SocialFiError::ArithmeticUnderflow)?;

    let clock = Clock::get()?;
    emit!(MemberKicked {
        group: ctx.accounts.group.key(),
        member: ctx.accounts.target_member.wallet,
        kicked_by: ctx.accounts.admin.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
