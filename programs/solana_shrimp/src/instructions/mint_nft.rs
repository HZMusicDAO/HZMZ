use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3, update_metadata_accounts_v2};

pub fn mint_nft(
    ctx: Context<MintNFT>,
    creator_key: Pubkey,
    uri: String,
    title: String,
    subdomain: String,
) -> Result<()> {
    // mint limit check , but we don't need this because collection

    let update_authority_pda_seed: &[u8] = subdomain.as_bytes();

    msg!("Transferring mint funds to treasury");
    msg!(&title);
    msg!(&uri);
    let uri_value = uri.clone();

    
    transfer_lamports(
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.contract_data.fee,
    )?;
    msg!(
        "Transferred lamport amount: {}",
        ctx.accounts.contract_data.fee
    );

    msg!("Initializing mint");
    let token_mint_id = mint_token(&ctx)?;
    msg!("Minted token id: {}", token_mint_id);

    msg!("Initializing metadata account");
    create_metadata_accounts(&ctx, creator_key, uri, title)?;
    msg!("Metadata account created !!!");

    msg!("Initializing master edition nft");
    create_master_edition(&ctx)?;
    msg!("Master edition nft minted !!!");

    // increase total minted amount
    ctx.accounts.contract_data.total_minted += 1;
    ctx.accounts.user_data.total_minted += 1;
    // save latest mint timestamp
    ctx.accounts.user_data.latest_mint_timestamp = Clock::get().unwrap().unix_timestamp as u32;

    let (_pda_key, _pda_key_bump) = Pubkey::find_program_address(&[update_authority_pda_seed], ctx.program_id);
 
     // account info
     let account_info = vec![
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
    ];
    invoke(
        &update_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint_authority.key(),
            // None,
            Some(_pda_key.key()),
            None,
            Some(true),
            None             
        ),
        account_info.as_slice(),
    )?;

    emit!(CollectionNFTMinted {
        nft_num: ctx.accounts.user_data.total_minted,
        holder: ctx.accounts.mint_authority.key(),
        uri: uri_value
    });
    
    Ok(())
}

fn transfer_lamports<'a>(from: AccountInfo<'a>, to: AccountInfo<'a>, amount: u64) -> Result<()> {
    let ix = system_instruction::transfer(&from.key(), &to.key(), amount);
    invoke(&ix, &[from, to])?;

    Ok(())
}

fn mint_token<'a>(ctx: &'a Context<MintNFT>) -> Result<&'a Pubkey> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let token_mint = ctx.accounts.mint.to_account_info();
    let token_mint_id = token_mint.key;
    let cpi_accounts = MintTo {
        mint: token_mint,
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    let mint_amount = 1;
    token::mint_to(cpi_ctx, mint_amount)?;

    Ok(token_mint_id)
}

fn create_metadata_accounts<'a>(
    ctx: &'a Context<MintNFT>,
    creator_key: Pubkey,
    uri: String,
    title: String,
) -> Result<()> {
    // token symbol
    let symbol = std::string::ToString::to_string("HZMZ");
    // creator
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: creator_key,
            verified: false,
            share: 5,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 95,
        },
    ];
    // account info
    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(
        &create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            title,
            symbol,
            uri,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
            None
        ),
        account_info.as_slice(),
    )?;

    Ok(())
}

fn create_master_edition(ctx: &Context<MintNFT>) -> Result<()> {
    // master edition info
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint_authority.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    mint: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    token_metadata_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    master_edition: UncheckedAccount<'info>,
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    #[account(init_if_needed,  seeds = [UserData::SEED, mint_authority.key().as_ref()], payer = mint_authority, bump, space = 8 + UserData::SPACE)]
    user_data: Account<'info, UserData>,
    /// CHECK:
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    treasury: UncheckedAccount<'info>,    
    /// CHECK:
    #[account(mut)]
    subdomains: UncheckedAccount<'info>,
}


#[event]
struct CollectionNFTMinted {
    nft_num: u16,
    holder: Pubkey,
    uri: String
}
