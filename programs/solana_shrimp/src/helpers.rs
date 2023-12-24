use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::{
    prelude::{AccountInfo, CpiContext, Program, Result},
    system_program::{self, System, Transfer as SolanaTransfer},
    ToAccountInfo,
};

// transfer lamports from on person to another without using pda signer
pub fn transfer_lamports<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &Program<'a, System>,
    lamports: u64,
) -> Result<()> {
    let cpi_accounts = SolanaTransfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };
    let cpi_program = system_program.to_account_info();

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    system_program::transfer(cpi_context, lamports)?;

    Ok(())
}

// transfer lamports from on person to another without using pda signer
pub fn transfer_lamports_from_treasury<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    program_id: &Pubkey,
    lamports: u64,
) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(&[b"hzmz_wallet".as_ref()], program_id);
    let asset_manager_seeds = &[b"hzmz_wallet".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    anchor_lang::solana_program::program::invoke_signed(                                                            
        &anchor_lang::solana_program::system_instruction::transfer( &from.key(),  &to.key(), lamports as u64),
        &[                          
            from.to_account_info(),
            to.to_account_info(),
        ],
        asset_manager_signer,
    )?;
    Ok(())
}
