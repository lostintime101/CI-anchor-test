use anchor_lang::prelude::*;

declare_id!("2E2tM56x3tKFqTvsqsCjo2ywyzV2jRat1dUyVD2pQAtQ");

#[program]
pub mod ci_anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
