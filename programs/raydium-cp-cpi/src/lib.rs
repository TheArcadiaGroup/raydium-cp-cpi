use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::TokenInterface};

#[cfg(feature = "devnet")]
declare_id!("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW");
#[cfg(not(feature = "devnet"))]
declare_id!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");

#[program]
pub mod raydium_cp_cpi {
    use super::*;

    pub fn initialize(
        _ctx: Context<Initialize>, 
        _init_amount_0: u64,
        _init_amount_1: u64,
        _open_time: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: Which config the pool belongs to.
    pub amm_config: UncheckedAccount<'info>,

    /// CHECK: pool vault and lp mint authority
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Initialize an account to store the pool state
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// CHECK: Token_0 mint, the key must smaller then token_1 mint.
    pub token_0_mint: UncheckedAccount<'info>,

    /// CHECK: Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: UncheckedAccount<'info>,

    /// pool lp mint
    #[account(mut)]
    pub lp_mint: UncheckedAccount<'info>,

    /// CHECK: payer token0 account
    #[account(mut)]
    pub creator_token_0: UncheckedAccount<'info>,

    /// CHECK: payer token1 account
    #[account(mut)]
    pub creator_token_1: UncheckedAccount<'info>,

    /// CHECK: creator lp token account
    #[account(mut)]
    pub creator_lp_token: UncheckedAccount<'info>,

    /// CHECK: Token_0 vault for the pool
    #[account(mut)]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool
    #[account(mut)]
    pub token_1_vault: UncheckedAccount<'info>,

    /// CHECK: create pool fee account
    #[account(mut)]
    pub create_pool_fee: UncheckedAccount<'info>,

    /// CHECK: an account to store oracle observations
    #[account(mut)]
    pub observation_state: UncheckedAccount<'info>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Spl token program or token program 2022
    pub token_0_program: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_1_program: Interface<'info, TokenInterface>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

