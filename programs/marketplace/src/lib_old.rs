use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FLiixxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod marketplace {
    use super::*;
    use anchor_spl::associated_token::AssociatedToken;
    use anchor_spl::token::{Mint, SetAuthority};
    use spl_token::instruction::AuthorityType;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee_percentage: u16,
        creator_fee_percentage: u16,
    ) -> Result<()> {
        require!(fee_percentage <= 1000, ErrorCode::InvalidFeePercentage); // Max 10%
        require!(
            creator_fee_percentage <= 500,
            ErrorCode::InvalidFeePercentage
        ); // Max 5%

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.creator_fee_percentage = creator_fee_percentage;
        marketplace.total_volume = 0;
        marketplace.total_listings = 0;
        marketplace.total_sales = 0;
        // These fields will be set later when we add escrow and treasury accounts
        // marketplace.escrow_account = ctx.accounts.escrow_account.key();
        // marketplace.treasury = ctx.accounts.treasury.key();

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
        marketplace.total_listings += 1;

        emit!(ComponentListed {
            component_id: component.component_id.clone(),
            creator: component.creator,
            price: component.price,
        });

        Ok(())
    }

    pub fn create_nft_listing(
        ctx: Context<CreateNFTListing>,
        price: u64,
        min_bid_increment: u64,
        auction_end: Option<i64>,
    ) -> Result<()> {
        require!(price > 0, ErrorCode::InvalidPrice);

        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.price = price;
        listing.min_bid_increment = min_bid_increment;
        listing.auction_end = auction_end;
        listing.highest_bid = 0;
        listing.highest_bidder = None;
        listing.is_active = true;
        listing.created_at = Clock::get()?.unix_timestamp;
        listing.listing_type = if auction_end.is_some() {
            ListingType::Auction
        } else {
            ListingType::FixedPrice
        };

        // Transfer NFT to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_nft_account.to_account_info(),
            to: ctx.accounts.escrow_nft_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_listings += 1;

        emit!(NFTListed {
            listing_id: listing.key(),
            seller: listing.seller,
            nft_mint: listing.nft_mint,
            price: listing.price,
            listing_type: listing.listing_type,
        });

        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.is_active, ErrorCode::ListingNotActive);
        require!(
            listing.listing_type == ListingType::Auction,
            ErrorCode::NotAnAuction
        );

        let current_time = Clock::get()?.unix_timestamp;
        if let Some(end_time) = listing.auction_end {
            require!(current_time < end_time, ErrorCode::AuctionEnded);
        }

        let min_bid = if listing.highest_bid > 0 {
            listing.highest_bid + listing.min_bid_increment
        } else {
            listing.price
        };
        require!(bid_amount >= min_bid, ErrorCode::BidTooLow);

        // Refund previous highest bidder if exists
        if let Some(previous_bidder) = listing.highest_bidder {
            let refund_accounts = Transfer {
                from: ctx.accounts.escrow_payment_account.to_account_info(),
                to: ctx.accounts.previous_bidder_account.to_account_info(),
                authority: ctx.accounts.marketplace.to_account_info(),
            };
            let seeds = &[b"marketplace".as_ref(), &[ctx.bumps.marketplace]];
            let signer = &[&seeds[..]];
            let refund_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                refund_accounts,
                signer,
            );
            token::transfer(refund_ctx, listing.highest_bid)?;
        }

        // Transfer new bid to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.bidder_payment_account.to_account_info(),
            to: ctx.accounts.escrow_payment_account.to_account_info(),
            authority: ctx.accounts.bidder.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, bid_amount)?;

        listing.highest_bid = bid_amount;
        listing.highest_bidder = Some(ctx.accounts.bidder.key());

        emit!(BidPlaced {
            listing_id: listing.key(),
            bidder: ctx.accounts.bidder.key(),
            bid_amount,
        });

        Ok(())
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let offer = &ctx.accounts.offer;

        require!(listing.is_active, ErrorCode::ListingNotActive);
        require!(
            listing.seller == ctx.accounts.seller.key(),
            ErrorCode::UnauthorizedSeller
        );
        require!(offer.is_active, ErrorCode::OfferNotActive);

        // Execute the trade
        execute_nft_sale(
            &ctx.accounts.token_program,
            &ctx.accounts.escrow_nft_account,
            &ctx.accounts.buyer_nft_account,
            &ctx.accounts.marketplace,
            &ctx.bumps,
            &ctx.accounts.offer_payment_account,
            &ctx.accounts.seller_payment_account,
            &ctx.accounts.treasury_account,
            &ctx.accounts.marketplace.to_account_info(),
            offer.amount,
            ctx.accounts.marketplace.fee_percentage,
            ctx.accounts.marketplace.creator_fee_percentage,
        )?;

        listing.is_active = false;
        offer.is_active = false;

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_sales += 1;
        marketplace.total_volume += offer.amount;

        emit!(SaleExecuted {
            listing_id: listing.key(),
            seller: listing.seller,
            buyer: offer.buyer,
            price: offer.amount,
        });

        Ok(())
    }

    pub fn purchase_nft(ctx: Context<PurchaseNFT>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.is_active, ErrorCode::ListingNotActive);
        require!(
            listing.listing_type == ListingType::FixedPrice,
            ErrorCode::NotFixedPrice
        );

        let marketplace = &ctx.accounts.marketplace;
        let total_price = listing.price;
        let platform_fee = total_price
            .checked_mul(marketplace.fee_percentage as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        let creator_fee = if let Some(creator) = ctx.accounts.creator.as_ref() {
            total_price
                .checked_mul(marketplace.creator_fee_percentage as u64)
                .unwrap()
                .checked_div(10000)
                .unwrap()
        } else {
            0
        };
        let seller_amount = total_price
            .checked_sub(platform_fee)
            .unwrap()
            .checked_sub(creator_fee)
            .unwrap();

        // Transfer seller amount to creator
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.creator_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, seller_amount)?;

        // Transfer fee to marketplace
        let cpi_accounts_fee = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.marketplace_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program_fee = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_fee = CpiContext::new(cpi_program_fee, cpi_accounts_fee);
        token::transfer(cpi_ctx_fee, platform_fee)?;

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
        seeds = [b"component", creator.key().as_ref()],
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
    pub creator_fee_percentage: u16,
    pub total_volume: u64,
    pub total_listings: u64,
    pub total_sales: u64,
    pub escrow_account: Pubkey,
    pub treasury: Pubkey,
}

impl Marketplace {
    pub const SIZE: usize = 32 + 2 + 2 + 8 + 8 + 8 + 32 + 32;
}

#[account]
pub struct NFTListing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub min_bid_increment: u64,
    pub auction_end: Option<i64>,
    pub highest_bid: u64,
    pub highest_bidder: Option<Pubkey>,
    pub is_active: bool,
    pub created_at: i64,
    pub listing_type: ListingType,
}

impl NFTListing {
    pub const SIZE: usize = 32 + 32 + 8 + 8 + 9 + 8 + 33 + 1 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ListingType {
    FixedPrice,
    Auction,
}

#[account]
pub struct Offer {
    pub buyer: Pubkey,
    pub listing: Pubkey,
    pub amount: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

impl Offer {
    pub const SIZE: usize = 32 + 32 + 8 + 1 + 8 + 9;
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

#[event]
pub struct NFTListed {
    pub listing_id: Pubkey,
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub listing_type: ListingType,
}

#[event]
pub struct BidPlaced {
    pub listing_id: Pubkey,
    pub bidder: Pubkey,
    pub bid_amount: u64,
}

#[event]
pub struct OfferMade {
    pub offer_id: Pubkey,
    pub listing_id: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct SaleExecuted {
    pub listing_id: Pubkey,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
}

#[event]
pub struct ListingCancelled {
    pub listing_id: Pubkey,
    pub seller: Pubkey,
}

#[event]
pub struct AuctionEnded {
    pub listing_id: Pubkey,
    pub winner: Pubkey,
    pub winning_bid: u64,
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
    #[msg("Listing not active")]
    ListingNotActive,
    #[msg("Not an auction")]
    NotAnAuction,
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("Bid too low")]
    BidTooLow,
    #[msg("Not fixed price listing")]
    NotFixedPrice,
    #[msg("Unauthorized seller")]
    UnauthorizedSeller,
    #[msg("Offer not active")]
    OfferNotActive,
    #[msg("Insufficient escrow balance")]
    InsufficientEscrowBalance,
    #[msg("Invalid NFT")]
    InvalidNFT,
    #[msg("Listing expired")]
    ListingExpired,
}
