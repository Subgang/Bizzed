use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod profit_share {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_tokens: u64) -> Result<()> {
        let profit_share = &mut ctx.accounts.profit_share;
        profit_share.total_tokens = total_tokens;
        profit_share.sme_wallet = *ctx.accounts.sme_wallet.key;
        Ok(())
    }

    pub fn distribute_profits(ctx: Context<DistributeProfits>, amount: u64) -> Result<()> {
        let profit_share = &ctx.accounts.profit_share;
        let total_tokens = profit_share.total_tokens;
        
        let transfer_amount = amount / total_tokens;

        for token_holder in &ctx.remaining_accounts {
            let mut token_holder_account = token_holder.try_borrow_mut_signer()?;

            let transfer_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sme_wallet.to_account_info(),
                    to: token_holder_account.to_account_info(),
                    authority: ctx.accounts.sme_wallet.to_account_info(),
                },
            );

            token::transfer(transfer_ctx, transfer_amount)?;
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ProfitShare {
    pub total_tokens: u64,
    pub sme_wallet: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub profit_share: Account<'info, ProfitShare>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub sme_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeProfits<'info> {
    #[account(mut)]
    pub profit_share: Account<'info, ProfitShare>,
    #[account(mut)]
    pub sme_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
}
