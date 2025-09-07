use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FLiixxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee_percentage: u16,
    ) -> Result<()> {
        require!(fee_percentage <= 10000, ErrorCode::InvalidFeePercentage);
        
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.total_volume = 0;
        marketplace.total_components = 0;
        
        Ok(())
    }

    pub fn list_component(
        ctx: Context<ListComponent>,
        component_id: String,
        price: u64,
        metadata_uri: String,
    ) -> Result<()> {
        require!(price > 0, ErrorCode::InvalidPrice);
        require!(component_id.len() <= 32, ErrorCode::ComponentIdTooLong);
        
        let component = &mut ctx.accounts.component;
        component.creator = ctx.accounts.creator.key();
        component.component_id = component_id;
        component.price = price;
        component.metadata_uri = metadata_uri;
        component.is_active = true;
        component.total_sales = 0;
        component.created_at = Clock::get()?.unix_timestamp;
        
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_components += 1;
        
        emit!(ComponentListed {
            component_id: component.component_id.clone(),
            creator: component.creator,
            price: component.price,
        });
        
        Ok(())
    }

    pub fn purchase_component(
        ctx: Context<PurchaseComponent>,
    ) -> Result<()> {
        let component = &ctx.accounts.component;
        require!(component.is_active, ErrorCode::ComponentNotActive);
        
        let marketplace = &ctx.accounts.marketplace;
        let fee_amount = component.price
            .checked_mul(marketplace.fee_percentage as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        let creator_amount = component.price
            .checked_sub(fee_amount)
            .unwrap();
        
        // Transfer to creator (70% in this case if fee is 30%)
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.creator_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, creator_amount)?;
        
        // Transfer fee to marketplace
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.marketplace_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, fee_amount)?;
        
        // Record purchase
        let purchase = &mut ctx.accounts.purchase;
        purchase.buyer = ctx.accounts.buyer.key();
        purchase.component_id = component.component_id.clone();
        purchase.price = component.price;
        purchase.purchased_at = Clock::get()?.unix_timestamp;
        
        // Update stats
        let component = &mut ctx.accounts.component;
        component.total_sales += 1;
        
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_volume += component.price;
        
        emit!(ComponentPurchased {
            component_id: component.component_id.clone(),
            buyer: ctx.accounts.buyer.key(),
            price: component.price,
        });
        
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
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListComponent<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Component::SIZE,
        seeds = [b"component", component.component_id.as_bytes()],
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
        init,
        payer = buyer,
        space = 8 + Purchase::SIZE,
        seeds = [b"purchase", buyer.key().as_ref(), component.component_id.as_bytes()],
        bump
    )]
    pub purchase: Account<'info, Purchase>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub marketplace_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub total_volume: u64,
    pub total_components: u64,
}

impl Marketplace {
    pub const SIZE: usize = 32 + 2 + 8 + 8;
}

#[account]
pub struct Component {
    pub creator: Pubkey,
    pub component_id: String,
    pub price: u64,
    pub metadata_uri: String,
    pub is_active: bool,
    pub total_sales: u64,
    pub created_at: i64,
}

impl Component {
    pub const SIZE: usize = 32 + 32 + 8 + 200 + 1 + 8 + 8;
}

#[account]
pub struct Purchase {
    pub buyer: Pubkey,
    pub component_id: String,
    pub price: u64,
    pub purchased_at: i64,
}

impl Purchase {
    pub const SIZE: usize = 32 + 32 + 8 + 8;
}

#[event]
pub struct ComponentListed {
    pub component_id: String,
    pub creator: Pubkey,
    pub price: u64,
}

#[event]
pub struct ComponentPurchased {
    pub component_id: String,
    pub buyer: Pubkey,
    pub price: u64,
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
}
