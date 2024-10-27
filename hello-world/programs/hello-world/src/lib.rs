use anchor_lang::prelude::*;

declare_id!("B8kDaBWptriUUieR61xzEwoZxFgL7qQSkDyBK6t1KgAC");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
