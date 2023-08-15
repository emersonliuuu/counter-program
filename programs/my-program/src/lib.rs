use anchor_lang::prelude::*;

declare_id!("FKkBRJhhYv4qxtr8QnpkwQhPN29TuArzWJptJeRVXepP");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, value: u64) -> Result<()> {
        let my_counter = &mut ctx.accounts.my_counter;
        my_counter.value = value;
        my_counter.bump = *ctx.bumps.get("my_counter").unwrap();

        Ok(())
    }

    pub fn add_even(ctx: Context<AddEven>, value: u64) -> Result<()> {
        require!(value % 2 == 0, ErrorCode::ValueIsNotEven);
        let my_counter = &mut ctx.accounts.my_counter;
        my_counter.value = my_counter.value.checked_add(value).unwrap();

        Ok(())
    }

    pub fn minus_odd(ctx: Context<MinusOdd>, value: u64) -> Result<()> {
        require!(value % 2 == 1, ErrorCode::ValueIsNotOdd);
        let my_counter = &mut ctx.accounts.my_counter;
        my_counter.value = my_counter.value.checked_sub(value).unwrap();

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // space: 8 (discriminator) + MyCounter::MAX_SIZE
    #[account(init_if_needed, payer = user, seeds = [b"my-counter", user.key().as_ref()], bump, space = 8 + MyCounter::MAX_SIZE)]
    pub my_counter: Account<'info, MyCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddEven<'info> {
    #[account(seeds = [b"my-counter", user.key().as_ref()], bump)]
    pub my_counter: Account<'info, MyCounter>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct MinusOdd<'info> {
    #[account(seeds = [b"my-counter", user.key().as_ref()], bump)]
    pub my_counter: Account<'info, MyCounter>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, seeds = [b"my-counter", user.key().as_ref()], bump, close = user)]
    pub my_counter: Account<'info, MyCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct MyCounter {
    pub value: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("value is not even")]
    ValueIsNotEven,
    #[msg("value is not odd")]
    ValueIsNotOdd,
}

impl MyCounter {
    // space: 8 (u64) + 1 (bump)
    const MAX_SIZE: usize = 8 + 1;
}
