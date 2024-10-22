use anchor_lang::prelude::*;

declare_id!("9o4VEobrYnMUAhanBsRxvrwu8wTfQFCpHeHN6YxX5FWX");

// #[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.as_key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state:Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.as_key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}


#[account]
pub struct VaultState {
    pub vault_bump:u8,
    pub state_bump:u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}


