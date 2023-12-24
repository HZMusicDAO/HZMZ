use crate::state::*;
use crate::helpers::*;
use anchor_lang::prelude::*;
use anchor_lang::{
    prelude::{Account, AccountInfo, Program, Result},
    system_program::System,
    ToAccountInfo,
};
use anchor_lang::solana_program::clock;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds = [ContractData::SEED], bump, payer = authority, space = 8 + ContractData::SPACE)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(init_if_needed, seeds = [TREASURY_SEED], bump, payer = authority, space = 0)]
    treasury: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [b"hzmz"], bump, payer = authority, space = 8 + 304)]
    pub game_state_account: Account<'info, GameState>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub treasury_account: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, amount: u64, dev1: Pubkey, total_supply: u64) -> Result<()> { // @BK: can remove mint_fee? its free
    let contract_data = &mut ctx.accounts.contract_data;

    contract_data.bump = *ctx.bumps.get("contract_data").unwrap();
    contract_data.treasury_bump = *ctx.bumps.get("treasury").unwrap();
    contract_data.authority = ctx.accounts.authority.key();
    contract_data.fee = 0; // @BK: set to 0 without needing arguments its free
    contract_data.total_supply = total_supply; 

    let game_state = &mut ctx.accounts.game_state_account;
    game_state.dev1 = dev1;
    game_state.seed_balance = amount as u128;

    // @BK: here should set game_state.premarket_end to be 3 days from now
    game_state.premarket_end = clock::Clock::get().unwrap().unix_timestamp as u64 + 259200; // 3 days * 24 hours * 60 minutes * 60 seconds

    emit!(Initialized { 
        seed_balance: amount as u128,
        bk_wallet: dev2,
        total_supply: total_supply,
        owner: ctx.accounts.authority.key()
     }); 
    
    Ok(())
}

#[event]
struct Initialized {
    total_supply: u64,
    bk_wallet: Pubkey,
    seed_balance: u128,
    owner: Pubkey
}
