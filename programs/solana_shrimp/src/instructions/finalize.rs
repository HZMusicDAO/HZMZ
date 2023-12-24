use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump, close = authority)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    treasury: UncheckedAccount<'info>,
    #[account(mut, address = contract_data.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
    let from = &ctx.accounts.treasury.to_account_info();
    let to = &ctx.accounts.authority.to_account_info();

    transfer_lamports(from, to, from.lamports())?;

    emit!(Finalized {
        authority: ctx.accounts.authority.key(),
    });

    Ok(())
}

fn transfer_lamports(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;

    Ok(())
}

#[event]
struct Finalized {
    authority: Pubkey,
}
