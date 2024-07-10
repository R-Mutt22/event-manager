use anchor_lang::prelude::*;

declare_id!("3LYBRHRpr959ApFB5FUxNdScgyMkpQXkvph4dCeKqvox");

#[program]
pub mod event_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
