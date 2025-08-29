use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

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

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.vlt_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;

        ctx.accounts
            .vlt
            .total_deposited
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // PDA signer seeds
        let seeds = &[b"vault", ctx.accounts.vlt.mint.as_ref(), &[ctx.bumps.vlt]];
        let signer_seeds = &[&seeds[..]];

        // CPI transfer (vault -> admin)
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vlt_ata.to_account_info(),
                to: ctx.accounts.admin_ata.to_account_info(),
                authority: ctx.accounts.vlt.to_account_info(),
            },
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        // Update state
        ctx.accounts.vlt.total_deposited = ctx.accounts.vlt.total_deposited.checked_sub(amount).unwrap();

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
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vlt: Account<'info, Vault>,

    #[account(
        mut,
        associated_token::mint = vlt.mint,
        associated_token::authority = vlt,
    )]
    pub vlt_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_account: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = vlt.mint,
        associated_token::authority = admin,
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", vlt.mint.as_ref()],
        bump
    )]
    pub vlt: Account<'info, Vault>,

    #[account(
        mut,
        associated_token::mint = vlt.mint,
        associated_token::authority = vlt,
    )]
    pub vlt_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub total_deposited: u64
}
