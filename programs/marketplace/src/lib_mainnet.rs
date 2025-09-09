use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("3RAeCuRqF9kGXyXwk9Lynj19fuxJJj5RXCga9KiihaKF");

// Your native FLII token mint address
pub const FLII_TOKEN_MINT: &str = "BMge7se4AqyTqEpcTSHURzA4YG9rNvmHscFEFJK9pump";

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee_percentage: u16,
    ) -> Result<()> {
        require!(fee_percentage <= 1000, ErrorCode::InvalidFeePercentage); // Max 10%

        // Verify the FLII token mint
        let expected_mint = FLII_TOKEN_MINT.parse::<Pubkey>().unwrap();
        require_keys_eq!(
            ctx.accounts.flii_token_mint.key(),
            expected_mint,
            ErrorCode::InvalidTokenMint
        );

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.total_volume = 0;
        marketplace.total_listings = 0;
        marketplace.total_sales = 0;
        marketplace.flii_token_mint = ctx.accounts.flii_token_mint.key();
        marketplace.treasury_wallet = ctx.accounts.treasury_wallet.key();

        Ok(())
    }

    pub fn list_component(
        ctx: Context<ListComponent>,
        component_id: String,
        price: u64,  // Price in FLII tokens
        metadata_uri: String,
    ) -> Result<()> {
        require!(price > 0, ErrorCode::InvalidPrice);
        require!(component_id.len() <= 32, ErrorCode::ComponentIdTooLong);

        let component = &mut ctx.accounts.component;
        component.creator = ctx.accounts.creator.key();
        component.component_id = component_id.clone();
        component.price = price;  // Price in FLII tokens
        component.metadata_uri = metadata_uri;
        component.is_active = true;
        component.total_sales = 0;
        component.total_rewards_earned = 0;  // Track FLII rewards earned
        component.created_at = Clock::get()?.unix_timestamp;

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_listings += 1;

        emit!(ComponentListed {
            component_id: component.component_id.clone(),
            creator: component.creator,
            price: component.price,
            token_mint: marketplace.flii_token_mint,
        });

        Ok(())
    }

    pub fn purchase_component(
        ctx: Context<PurchaseComponent>,
    ) -> Result<()> {
        let component = &mut ctx.accounts.component;
        require!(component.is_active, ErrorCode::ComponentNotActive);
        
        // Verify FLII token mint
        require_keys_eq!(
            ctx.accounts.flii_token_mint.key(),
            ctx.accounts.marketplace.flii_token_mint,
            ErrorCode::InvalidTokenMint
        );

        let marketplace = &mut ctx.accounts.marketplace;
        let total_price = component.price;  // Price in FLII tokens
        
        // Calculate fees and rewards
        let platform_fee = total_price
            .checked_mul(marketplace.fee_percentage as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        
        // Calculate staking rewards (e.g., 2% bonus for using FLII)
        let staking_reward = total_price
            .checked_mul(200)  // 2% reward
            .unwrap()
            .checked_div(10000)
            .unwrap();
            
        let creator_amount = total_price
            .checked_sub(platform_fee)
            .unwrap()
            .checked_add(staking_reward)  // Creator gets base + reward
            .unwrap();

        // Transfer FLII tokens to creator (including rewards)
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_flii_token_account.to_account_info(),
            to: ctx.accounts.creator_flii_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, creator_amount)?;

        // Transfer platform fee in FLII to marketplace treasury
        if platform_fee > 0 {
            let cpi_accounts_fee = Transfer {
                from: ctx.accounts.buyer_flii_token_account.to_account_info(),
                to: ctx.accounts.treasury_flii_token_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_program_fee = ctx.accounts.token_program.to_account_info();
            let cpi_ctx_fee = CpiContext::new(cpi_program_fee, cpi_accounts_fee);
            token::transfer(cpi_ctx_fee, platform_fee)?;
        }

        // Update stats
        component.total_sales += 1;
        component.total_rewards_earned += staking_reward;
        marketplace.total_volume += total_price;
        marketplace.total_sales += 1;

        emit!(ComponentPurchased {
            component_id: component.component_id.clone(),
            buyer: ctx.accounts.buyer.key(),
            price: component.price,
            rewards_earned: staking_reward,
            token_mint: marketplace.flii_token_mint,
        });

        Ok(())
    }

    pub fn delist_component(
        ctx: Context<DelistComponent>,
    ) -> Result<()> {
        let component = &mut ctx.accounts.component;
        require!(
            component.creator == ctx.accounts.creator.key(),
            ErrorCode::UnauthorizedCreator
        );
        
        component.is_active = false;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::SIZE,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        constraint = flii_token_mint.key() == FLII_TOKEN_MINT.parse::<Pubkey>().unwrap()
    )]
    pub flii_token_mint: Account<'info, Mint>,
    /// CHECK: Treasury wallet for collecting fees
    pub treasury_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(component_id: String)]
pub struct ListComponent<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Component::SIZE,
        seeds = [b"component", component_id.as_bytes()],
        bump
    )]
    pub component: Account<'info, Component>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseComponent<'info> {
    #[account(mut)]
    pub component: Account<'info, Component>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        constraint = flii_token_mint.key() == FLII_TOKEN_MINT.parse::<Pubkey>().unwrap()
    )]
    pub flii_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        constraint = buyer_flii_token_account.mint == flii_token_mint.key(),
        constraint = buyer_flii_token_account.owner == buyer.key()
    )]
    pub buyer_flii_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = creator_flii_token_account.mint == flii_token_mint.key()
    )]
    pub creator_flii_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = treasury_flii_token_account.mint == flii_token_mint.key(),
        constraint = treasury_flii_token_account.owner == marketplace.treasury_wallet
    )]
    pub treasury_flii_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct DelistComponent<'info> {
    #[account(mut)]
    pub component: Account<'info, Component>,
    pub creator: Signer<'info>,
}

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub total_volume: u64,
    pub total_listings: u64,
    pub total_sales: u64,
    pub flii_token_mint: Pubkey,
    pub treasury_wallet: Pubkey,
}

impl Marketplace {
    pub const SIZE: usize = 32 + 2 + 8 + 8 + 8 + 32 + 32;
}

#[account]
pub struct Component {
    pub creator: Pubkey,
    pub component_id: String,
    pub price: u64,  // Price in FLII tokens
    pub metadata_uri: String,
    pub is_active: bool,
    pub total_sales: u64,
    pub total_rewards_earned: u64,  // Total FLII rewards earned
    pub created_at: i64,
}

impl Component {
    pub const SIZE: usize = 32 + 36 + 8 + 200 + 1 + 8 + 8 + 8;
}

#[event]
pub struct ComponentListed {
    pub component_id: String,
    pub creator: Pubkey,
    pub price: u64,  // Price in FLII tokens
    pub token_mint: Pubkey,
}

#[event]
pub struct ComponentPurchased {
    pub component_id: String,
    pub buyer: Pubkey,
    pub price: u64,  // Price in FLII tokens
    pub rewards_earned: u64,  // FLII rewards earned
    pub token_mint: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Component ID too long")]
    ComponentIdTooLong,
    #[msg("Component not active")]
    ComponentNotActive,
    #[msg("Unauthorized creator")]
    UnauthorizedCreator,
    #[msg("Invalid token mint - must use FLII token")]
    InvalidTokenMint,
    #[msg("Insufficient FLII token balance")]
    InsufficientTokenBalance,
}
