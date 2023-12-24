use anchor_lang::prelude::*;

#[account]
pub struct ContractData {
    pub bump: u8,
    pub treasury_bump: u8,
    pub authority: Pubkey,
    pub fee: u64,
    pub total_minted: u64, // 65536
    pub total_supply: u64, // 65536
}

impl ContractData {
    pub const SPACE: usize = 1 + 1 + 52 + 8 + 2;
    pub const SEED: &'static [u8] = b"contractdata";
    pub const LIMIT: u16 = 3; // MAX mint limit
}

#[account]
pub struct UserInfo {
    pub data_info: String,
}

impl UserInfo {
    pub const SPACE: usize = 10000;
    pub const SEED: &'static [u8] = b"user_info";
}

#[account]
pub struct UserData {
    pub total_minted: u16,
    pub latest_mint_timestamp: u32,
}
impl UserData {
    pub const SPACE: usize = 2 + 4;
    pub const SEED: &'static [u8] = b"userdata";
}

pub const TREASURY_SEED: &[u8] = b"treasury";

pub const PSN: u128 = 10000;
pub const PSNH: u128 = 5000;
pub const FEE: u128 = 10; // 4% to devs, 6% to pre-market

#[account]
pub struct SubDomainData {
    pub subdomain: String,
}


#[account]
pub struct GameState {
    pub dev_balance: u128, // Dev fee balance
    pub dev1: Pubkey,  // Dev 1
    pub collecton_key: Pubkey,
}
