use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

pub mod instructions;
pub mod state;
pub mod math;

use state::*;

// TODO: Replace with real program ID:
// 1. Generate keypair: solana-keygen new -o target/deploy/clmm_amm-keypair.json
// 2. Get pubkey: solana-keygen pubkey target/deploy/clmm_amm-keypair.json
// 3. Update this line with the pubkey
declare_id!("11111111111111111111111111111111");

#[program]
pub mod clmm_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        tick_spacing: u16,
        initial_sqrt_price_x64: u128,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, tick_spacing, initial_sqrt_price_x64)
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        instructions::swap::handler(ctx, amount, other_amount_threshold, sqrt_price_limit_x64, is_base_input)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        liquidity: u128,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        instructions::open_position::handler(ctx, liquidity, tick_lower_index, tick_upper_index)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid tick spacing - must be divisible by pool tick spacing")]
    InvalidTickSpacing,
    #[msg("Invalid tick range - lower must be less than upper")]
    InvalidTickRange,
    #[msg("Tick account mismatch - passed account doesn't match expected tick index")]
    TickAccountMismatch,
    #[msg("Invalid tick account")]
    InvalidTickAccount,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Pool::LEN,
        seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = mint_a,
        token::authority = pool,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = mint_b,
        token::authority = pool,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub payer_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Position::LEN,
        seeds = [b"position", pool.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub tick_lower: Account<'info, Tick>,

    #[account(mut)]
    pub tick_upper: Account<'info, Tick>,

    #[account(mut)]
    pub payer_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
