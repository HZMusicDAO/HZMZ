use crate::state::*;
use anchor_lang::prelude::*;

pub fn set_collection(
    ctx: Context<SetCollection>,
    collecton_key: Pubkey,
    premarket_end: u64
) -> Result<()> {
    let game_account = &mut ctx.accounts.game_account;
    game_account.collecton_key = collecton_key;
    game_account.premarket_end = premarket_end;
    Ok(())
}

#[derive(Accounts)]
pub struct SetCollection<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut, seeds = [b"hzmz"], bump )]
    game_account: Account<'info, GameState>,
}