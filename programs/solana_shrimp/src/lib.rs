use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod helpers;

declare_id!("B9n6kvLkZGATZCvL6H2ujjzQE28jvbT2k6N1vKt9hC6L");

#[program]
pub mod solana_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, dev1: Pubkey, dev2: Pubkey, dev3: Pubkey, total_supply: u64) -> Result<()> {
        instructions::initialize(ctx, amount, dev1, dev2, dev3, total_supply)
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        instructions::finalize(ctx)
    }

    pub fn update_fee(ctx: Context<UpdateFee>, mint_fee: u64) -> Result<()> {
        instructions::update_fee(ctx, mint_fee)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
        subdomain: String,
    ) -> Result<()> {
        instructions::mint_nft(ctx, creator_key, uri, title, subdomain)
    }

    pub fn mint_nftcollection(
        ctx: Context<MintNFTCollection>,
        creator_key: Pubkey,
        uri: String,
        title: String,
        subdomain: String,
    ) -> Result<()> {
        instructions::mint_nftcollection(ctx, creator_key, uri, title, subdomain)
    }

    pub fn set_collection(ctx: Context<SetCollection>, collection_key: Pubkey, premarket_end: u64) -> Result<()> {
        instructions::set_collection(ctx, collection_key, premarket_end)
    }

    pub fn dev_withdraw(ctx: Context<DevWithdraw>) -> Result<()> {
        instructions::dev_withdraw(ctx)
    }
}
