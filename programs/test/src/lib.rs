use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("Csj4WkNPZa7wtoT8yrxLPouB98gprmY2CgoKETntzEUW");

#[program]
pub mod vault {
  use anchor_spl::token;

use super::*;

    pub fn initialize_vault_account(ctx: Context<InitializeVaultAccount>) -> Result<()> {

        let vault = &mut ctx.accounts.vlt;
        vault.admin = ctx.accounts.user.key();
        vault.mint = ctx.accounts.mint.key();
        vault.total_deposited = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositAccount>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from : ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.vault_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        let vault = &mut ctx.accounts.vlt;
        vault.total_deposited = vault.total_deposited.checked_add(amount).unwrap();

        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeVaultAccount<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
    )]
    pub vlt: Account<'info, Vault>,

    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct DepositAccount<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// CHECK: 
    #[account(mut)]
    pub user_ata: AccountInfo<'info>,

    #[account(mut)]
    pub vlt: Account<'info, Vault>,

    /// CHECK:
    #[account(mut)]
    pub vault_ata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub total_deposited: u64,
}
