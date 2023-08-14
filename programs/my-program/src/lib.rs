use anchor_lang::prelude::*;

declare_id!("FKkBRJhhYv4qxtr8QnpkwQhPN29TuArzWJptJeRVXepP");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, value: u64) -> Result<()> {
        let my_counter = &mut ctx.accounts.my_counter;
        my_counter.value = value;

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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // space: 8 (discriminator) + 8 (u64)
    #[account(init, payer = user, space = 8 + 8)]
    pub my_counter: Account<'info, MyCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddEven<'info> {
    #[account(mut)]
    pub my_counter: Account<'info, MyCounter>,
}

#[derive(Accounts)]
pub struct MinusOdd<'info> {
    #[account(mut)]
    pub my_counter: Account<'info, MyCounter>,
}

#[account]
pub struct MyCounter {
    pub value: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("value is not even")]
    ValueIsNotEven,
    #[msg("value is not odd")]
    ValueIsNotOdd,
}
