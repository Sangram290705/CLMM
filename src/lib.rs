use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod clmm_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        tick_spacing: u16,
        initial_sqrt_price_x64: u128,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.bump = ctx.bumps.pool;
        pool.mint_a = ctx.accounts.mint_a.key();
        pool.mint_b = ctx.accounts.mint_b.key();
        pool.vault_a = ctx.accounts.vault_a.key();
        pool.vault_b = ctx.accounts.vault_b.key();
        pool.sqrt_price_x64 = initial_sqrt_price_x64;
        pool.liquidity = 0;
        pool.tick_current = 0;
        pool.tick_spacing = tick_spacing;
        pool.fee_growth_global_a = 0;
        pool.fee_growth_global_b = 0;
        pool.protocol_fee_a = 0;
        pool.protocol_fee_b = 0;
        pool.padding = [0; 8];
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        _other_amount_threshold: u64,
        _sqrt_price_limit_x64: u128,
        _is_base_input: bool,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        if amount > 0 {
            if ctx.accounts.payer_token_a.amount > 0 {
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.payer_token_a.to_account_info(),
                        to: ctx.accounts.vault_a.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info(),
                    },
                );
                token::transfer(cpi_ctx, amount)?;
            }
        }

        pool.sqrt_price_x64 = pool.sqrt_price_x64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 1 + 32 + 32 + 32 + 32 + 16 + 16 + 4 + 2 + 16 + 16 + 8 + 8 + 8 * 8,
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

    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub bump: u8,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick_current: i32,
    pub tick_spacing: u16,
    pub fee_growth_global_a: u128,
    pub fee_growth_global_b: u128,
    pub protocol_fee_a: u64,
    pub protocol_fee_b: u64,
    pub padding: [u64; 8],
}
