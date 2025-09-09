pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("TokenProgramID11111111111111111111111111111");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
