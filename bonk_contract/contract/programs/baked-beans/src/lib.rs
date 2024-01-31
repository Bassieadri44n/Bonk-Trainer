use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("25aYz6xiWAZXQrvGdV42QRFthdUoEzB7VpcwHWp9kv5x");
#[program]
pub mod baked_beans {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
        initialize::handle(ctx, new_authority)
    }

    pub fn buy_bonks(ctx: Context<BuyBonks>, amount: u64) -> Result<()> {
        buy_bonks::handle(ctx, amount)
    }

    pub fn sell_bonks(ctx: Context<SellBonks>) -> Result<()> {
        sell_bonks::handle(ctx)
    }

    pub fn hatch_bonks(ctx: Context<HatchBonks>) -> Result<()> {
        hatch_bonks::handle(ctx)
    }

    pub fn start_mine(ctx: Context<StartMine>) -> Result<()> {
        start_mine::handle(ctx)
    }

    pub fn set_treasury(ctx: Context<SetTreasury>, key: Pubkey) -> Result<()> {
        set_treasury::handle(ctx, key)
    }
}
