use anchor_lang::prelude::*;

declare_id!("6uLH2QFj6dxRQsxSeyr3xvt24rMRQc4Wx1T2cX5iNVa1");

#[program]
pub mod kyrx {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
