use anchor_lang::prelude::*;

#[account]
pub struct Tick {
    pub bump: u8,
    pub pool: Pubkey,
    pub tick_index: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub padding: [u64; 8],
}

impl Tick {
    pub const LEN: usize = 8 + 1 + 32 + 4 + 16 + 16 + 16 + 16 + 8 * 8;

    /// Returns the PDA seeds for a tick account
    pub fn seeds(pool: Pubkey, tick_index: i32) -> Vec<Vec<u8>> {
        vec![
            b"tick".to_vec(),
            pool.as_ref().to_vec(),
            tick_index.to_le_bytes().to_vec(),
        ]
    }
}
