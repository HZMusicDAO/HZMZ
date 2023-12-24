use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use crate::helpers::*;

#[derive(Accounts)]
pub struct DevWithdraw<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(mut)]
    treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub game_state_account: Account<'info, GameState>,
    /// CHECK:
    #[account(mut)]
    admin_account: UncheckedAccount<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn dev_withdraw(ctx: Context<DevWithdraw>) -> Result<()> {
    // Warning: code is not safe !!!
    let game_state = &mut ctx.accounts.game_state_account;
    let from = &ctx.accounts.treasury.to_account_info();

    let dev_balance = game_state.dev_balance; // Total dev balance
    let bk = game_state.dev2;       // @admin   wallet

    let available_amount_to_withdraw = 
        **ctx.accounts.treasury.to_account_info().lamports.borrow(); // treasury total balance

    msg!("The dev_balance is {}", dev_balance);
    msg!("The available_amount_to_withdraw is {}", available_amount_to_withdraw);

    require!(
        bk.eq(&ctx.accounts.admin_account.key()),
        self::CustomErrors::WrongDevWallet
    );

    require!(
        available_amount_to_withdraw > 0,
        self::CustomErrors::RequestRentSol
    );
    
    require!(
        dev_balance <= available_amount_to_withdraw as u128,
        self::CustomErrors::InsufficientFunds
    );

    // Calculate shares
    let admin_amount = dev_balance // dev_balance

    game_state.dev_balance = 0;

    // Send from treasury to the @admin
    transfer_lamports_from_treasury(
        from,
        &ctx.accounts.admin_account,
        ctx.program_id,
        admin_amount as u64
    )?;

    emit!(DevWithdrawn {
        dev_balance: dev_balance as u64,
        admin_amount: admin_amount as u64,
        authority: ctx.accounts.authority.key()
    });

    Ok(())
}

#[event]
struct DevWithdrawn {
    dev_balance: u64,
    admin_amount: u64,
    authority: Pubkey,
}
