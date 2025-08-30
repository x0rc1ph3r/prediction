#![allow(clippy::result_large_err)]

pub mod states;
use crate::states::*;
pub mod errors;
use crate::errors::*;
pub mod contexts;
use crate::contexts::*;
pub mod instructions;
use crate::instructions::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("CNd5sdD29D2Vip3d2qhzhimd2wXDDqSgpQS2vjP6u9W6");

pub const MAXIMUM_AGE: u64 = 1000; // One minute
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD
pub const TREASURER: Pubkey =
    solana_program::pubkey!("i2tZJMMTqrcYv53qdLFsouL1JQPWgKiTfZ6sRDfk7nL");

#[program]
pub mod betting {
    use super::*;

    pub fn create_treasury(ctx: Context<CreateTreasury>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.treasurer = TREASURER;
        treasury.bump = ctx.bumps.treasury;
        Ok(())
    }

    pub fn set_treasury(ctx: Context<SetTreasury>, new_treasurer: Pubkey) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.treasurer = new_treasurer;
        Ok(())
    }

    pub fn start_round(ctx: Context<StartRound>, id: u64) -> Result<()> {
        instruction_create_round(ctx, id)
    }

    pub fn stop_round(ctx: Context<StopRound>, id: u64) -> Result<()> {
        instruction_stop_round(ctx, id)
    }

    pub fn bet(ctx: Context<Bet>, id: u64, amount: u64, prediction_up: bool) -> Result<()> {
        instruction_bet(ctx, id, amount, prediction_up)
    }

    pub fn claim(ctx: Context<Claim>, id: u64) -> Result<()> {
        instruction_claim(ctx, id)
    }
}
