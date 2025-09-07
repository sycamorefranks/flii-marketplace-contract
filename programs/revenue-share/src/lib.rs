use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FLiiRSxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod revenue_share {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        creator_share: u16,
        platform_share: u16,
    ) -> Result<()> {
        require!(
            creator_share + platform_share == 10000,
            ErrorCode::InvalidShares
        );
        
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.creator_share = creator_share;
        pool.platform_share = platform_share;
        pool.total_distributed = 0;
        
        Ok(())
    }

    pub fn distribute_revenue(
        ctx: Context<DistributeRevenue>,
        amount: u64,
    ) -> Result<()> {
        let pool = &ctx.accounts.pool;
        
        let creator_amount = amount
            .checked_mul(pool.creator_share as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
            
        let platform_amount = amount
            .checked_sub(creator_amount)
            .unwrap();
        
        // Transfer to creator
        let cpi_accounts = Transfer {
            from: ctx.accounts.source_account.to_account_info(),
            to: ctx.accounts.creator_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, creator_amount)?;
        
        // Transfer to platform
        let cpi_accounts = Transfer {
            from: ctx.accounts.source_account.to_account_info(),
            to: ctx.accounts.platform_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, platform_amount)?;
        
        let pool = &mut ctx.accounts.pool;
        pool.total_distributed += amount;
        
        emit!(RevenueDistributed {
            amount,
            creator_amount,
            platform_amount,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RevenuePool::SIZE,
        seeds = [b"revenue_pool"],
        bump
    )]
    pub pool: Account<'info, RevenuePool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRevenue<'info> {
    #[account(mut)]
    pub pool: Account<'info, RevenuePool>,
    #[account(mut)]
    pub source_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub creator_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub platform_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RevenuePool {
    pub authority: Pubkey,
    pub creator_share: u16,
    pub platform_share: u16,
    pub total_distributed: u64,
}

impl RevenuePool {
    pub const SIZE: usize = 32 + 2 + 2 + 8;
}

#[event]
pub struct RevenueDistributed {
    pub amount: u64,
    pub creator_amount: u64,
    pub platform_amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid share percentages")]
    InvalidShares,
}
