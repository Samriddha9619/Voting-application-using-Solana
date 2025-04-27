use anchor_lang::prelude::*;

declare_id!("GyfWwFoReD5fJJhF1A5QCo7w1nYPxV7y9vYGfGmcGufq");

#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
