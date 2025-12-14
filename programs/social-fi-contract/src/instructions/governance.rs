use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

// ==================== Stake Tokens ====================

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        init,
        payer = staker,
        space = StakePosition::LEN,
        seeds = [STAKE_POSITION_SEED, staker.key().as_ref()],
        bump
    )]
    pub stake_position: Account<'info, StakePosition>,
    
    #[account(mut)]
    pub staker: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn stake_tokens(
    ctx: Context<StakeTokens>,
    amount: u64,
    lock_period: u64,
) -> Result<()> {
    require!(amount > 0, SocialFiError::InvalidAmount);
    
    // Validate lock period
    require!(
        lock_period == LOCK_0_DAYS ||
        lock_period == LOCK_30_DAYS ||
        lock_period == LOCK_90_DAYS ||
        lock_period == LOCK_180_DAYS ||
        lock_period == LOCK_365_DAYS,
        SocialFiError::InvalidLockPeriod
    );

    let stake_position = &mut ctx.accounts.stake_position;
    let clock = Clock::get()?;
    
    // Calculate unlock time
    let lock_seconds = lock_period
        .checked_mul(SECONDS_PER_DAY as u64)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    let unlocks_at = clock
        .unix_timestamp
        .checked_add(lock_seconds as i64)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    // Calculate voting power
    let voting_power = StakePosition::calculate_voting_power(amount, lock_period)?;

    // Initialize stake position
    stake_position.staker = ctx.accounts.staker.key();
    stake_position.amount = amount;
    stake_position.staked_at = clock.unix_timestamp;
    stake_position.lock_period = lock_period;
    stake_position.unlocks_at = unlocks_at;
    stake_position.rewards = 0;
    stake_position.voting_power = voting_power;
    stake_position.bump = ctx.bumps.stake_position;

    // NOTE: In production, this would transfer tokens to a vault
    // For now, we're just tracking the stake

    emit!(TokensStaked {
        staker: ctx.accounts.staker.key(),
        amount,
        lock_period,
        voting_power,
        unlocks_at,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Unstake Tokens ====================

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(
        mut,
        seeds = [STAKE_POSITION_SEED, staker.key().as_ref()],
        bump = stake_position.bump,
        close = staker
    )]
    pub stake_position: Account<'info, StakePosition>,
    
    #[account(mut)]
    pub staker: Signer<'info>,
}

pub fn unstake_tokens(ctx: Context<UnstakeTokens>) -> Result<()> {
    let stake_position = &ctx.accounts.stake_position;
    let clock = Clock::get()?;

    require!(
        stake_position.is_unlocked(clock.unix_timestamp),
        SocialFiError::TokensLocked
    );

    // Calculate rewards
    let rewards = stake_position.calculate_rewards(clock.unix_timestamp)?;
    let _total_return = stake_position
        .amount
        .checked_add(rewards)
        .ok_or(SocialFiError::ArithmeticOverflow)?;

    // NOTE: In production, this would transfer tokens from vault back to user
    
    emit!(TokensUnstaked {
        staker: ctx.accounts.staker.key(),
        amount: stake_position.amount,
        rewards,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Create Proposal ====================

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = proposer,
        space = Proposal::LEN,
        seeds = [PROPOSAL_SEED, proposer.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [STAKE_POSITION_SEED, proposer.key().as_ref()],
        bump = stake_position.bump,
        constraint = stake_position.voting_power >= MIN_VOTING_POWER @ SocialFiError::InsufficientVotingPower
    )]
    pub stake_position: Account<'info, StakePosition>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    category: u8,
    execution_delay: i64,
) -> Result<()> {
    require!(
        title.len() <= MAX_TITLE_LENGTH,
        SocialFiError::ProposalTitleTooLong
    );
    require!(
        description.len() <= MAX_DESCRIPTION_LENGTH,
        SocialFiError::ProposalDescriptionTooLong
    );
    require!(category <= 3, SocialFiError::InvalidProposalCategory);
    require!(
        execution_delay >= MIN_EXECUTION_DELAY,
        SocialFiError::ExecutionDelayNotMet
    );

    let clock = Clock::get()?;
    
    let voting_ends_at = clock
        .unix_timestamp
        .checked_add(VOTING_PERIOD)
        .ok_or(SocialFiError::ArithmeticOverflow)?;
    
    // Calculate quorum (10% of total staked)
    // In production, this would query total staked from a global state account
    let quorum_required = ctx.accounts.stake_position.voting_power
        .checked_mul(10)
        .ok_or(SocialFiError::ArithmeticOverflow)?; // Simplified for demo
    
    let proposal_key = ctx.accounts.proposal.key();
    let proposal = &mut ctx.accounts.proposal;

    proposal.id = proposal_key;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.title = title.clone();
    proposal.description = description;
    proposal.category = category;
    proposal.status = 0; // active
    proposal.created_at = clock.unix_timestamp;
    proposal.voting_ends_at = voting_ends_at;
    proposal.execution_delay = execution_delay;
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    proposal.votes_abstain = 0;
    proposal.quorum_required = quorum_required;
    proposal.executed_at = None;
    proposal.bump = ctx.bumps.proposal;

    emit!(ProposalCreated {
        proposal: ctx.accounts.proposal.key(),
        proposer: ctx.accounts.proposer.key(),
        title,
        category,
        voting_ends_at,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Cast Vote ====================

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.proposer.as_ref(), proposal.title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = Vote::LEN,
        seeds = [VOTE_SEED, proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    
    #[account(
        seeds = [STAKE_POSITION_SEED, voter.key().as_ref()],
        bump = stake_position.bump
    )]
    pub stake_position: Account<'info, StakePosition>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn cast_vote(ctx: Context<CastVote>, vote_type: u8) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    let clock = Clock::get()?;

    require!(
        proposal.is_active(clock.unix_timestamp),
        SocialFiError::VotingPeriodEnded
    );
    require!(vote_type <= 2, SocialFiError::InvalidProposalCategory);

    let voting_power = ctx.accounts.stake_position.voting_power;
    
    // Record vote
    let vote = &mut ctx.accounts.vote;
    vote.proposal = ctx.accounts.proposal.key();
    vote.voter = ctx.accounts.voter.key();
    vote.vote_type = vote_type;
    vote.voting_power = voting_power;
    vote.voted_at = clock.unix_timestamp;
    vote.bump = ctx.bumps.vote;

    // Update proposal vote counts
    let proposal = &mut ctx.accounts.proposal;
    match vote_type {
        0 => {
            proposal.votes_for = proposal
                .votes_for
                .checked_add(voting_power)
                .ok_or(SocialFiError::ArithmeticOverflow)?;
        }
        1 => {
            proposal.votes_against = proposal
                .votes_against
                .checked_add(voting_power)
                .ok_or(SocialFiError::ArithmeticOverflow)?;
        }
        2 => {
            proposal.votes_abstain = proposal
                .votes_abstain
                .checked_add(voting_power)
                .ok_or(SocialFiError::ArithmeticOverflow)?;
        }
        _ => return Err(error!(SocialFiError::InvalidProposalCategory)),
    }

    emit!(VoteCast {
        proposal: ctx.accounts.proposal.key(),
        voter: ctx.accounts.voter.key(),
        vote_type,
        voting_power,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Execute Proposal ====================

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.proposer.as_ref(), proposal.title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub executor: Signer<'info>,
}

pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    let clock = Clock::get()?;

    require!(
        !proposal.is_active(clock.unix_timestamp),
        SocialFiError::VotingPeriodActive
    );
    require!(
        proposal.has_passed(),
        SocialFiError::ProposalNotPassed
    );
    require!(
        proposal.can_execute(clock.unix_timestamp),
        SocialFiError::ExecutionDelayNotMet
    );

    let proposal_key = ctx.accounts.proposal.key();
    let votes_for = ctx.accounts.proposal.votes_for;
    let votes_against = ctx.accounts.proposal.votes_against;
    
    let proposal = &mut ctx.accounts.proposal;
    
    proposal.status = 3; // executed
    proposal.executed_at = Some(clock.unix_timestamp);

    emit!(ProposalExecuted {
        proposal: proposal_key,
        executor: ctx.accounts.executor.key(),
        votes_for,
        votes_against,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
