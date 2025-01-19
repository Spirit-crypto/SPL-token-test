use anchor_lang::prelude::*;

declare_id!("EGgoFhYpxoPuPBhWLCyTGRptKgrFkd6zutXfWnG66naX");

#[program]
pub mod spl_token_5jyj {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
