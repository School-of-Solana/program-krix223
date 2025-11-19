use anchor_lang::prelude::*;

declare_id!("REPLACE_WITH_YOUR_PROGRAM_ID"); // <-- replace with your deployed program ID

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.owner = *ctx.accounts.user.key;
        counter.count = 0;
        counter.total_increments = 0;
        counter.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        counter.total_increments += 1;
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", counter.owner.as_ref()], bump)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut, seeds = [b"counter", counter.owner.as_ref()], bump, has_one = owner)]
    pub counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Counter {
    pub owner: Pubkey,
    pub count: u64,
    pub total_increments: u64,
    pub created_at: i64,
}
