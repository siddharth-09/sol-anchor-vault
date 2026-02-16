use anchor_lang::prelude::*;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
declare_id!("F7sbYCvoQVkJaAidq11SnANGj5Y8puwReQV55a5BW1qV");

/*
What to do
1. Initialize a empty anchor program.
2. Wrte All the instructions: Initiialize, 
deposit, 
withdraw, 
close.
3. Write tests for each instruction.
*/
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized user")]
    Unauthorized,
    #[msg("Unable to close Account : Empty the balance")]
    Empty,
    #[msg("Unable to transfer sol to Vault : User doesnt have Balance")]
    InsufficientBalance,
    #[msg("Unable to Withdraw : Vault dont have Balance")]
    VaultInsufficientBalance

}

#[program]
pub mod vault_program {


    use super::*;

    //Initialize
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let rent_exempt = Rent::get()?.minimum_balance(ctx.accounts.vault.to_account_info().data_len());
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from : ctx.accounts.user.to_account_info(),
            to : ctx.accounts.vault.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, rent_exempt)?;

        ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;
        ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;

        Ok(())
    }
    //Deposit
    pub fn deposit(ctx: Context<Deposit>,amount : u64)-> Result<()>{
        let vault = &ctx.accounts.vault;
        let user = &mut ctx.accounts.user;
        // if vault.vault_authority != user.key(){
        //     return Err(ErrorCode::Unauthorized.into());
        // }
        if user.lamports() < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from : user.to_account_info(),
            to : vault.to_account_info()
        };
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(ctx,amount)?;
        Ok(())
    }
    //Withdraw
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()>{
        let vault = &ctx.accounts.vault;
        let user = &ctx.accounts.user;
        let balance = vault.to_account_info().lamports();
        // if vault.vault_authority != user.key(){
        //     return Err(ErrorCode::Unauthorized.into());
        // }
        if vault.to_account_info().lamports() == 0{
            return Err(ErrorCode::VaultInsufficientBalance.into());
        }
        let vaultState = &ctx.accounts.vault_state;
        let vaultStateKey = vaultState.key();
        let signer_seeds: [&[&[u8]]; 1]= [&[
            b"vault",
            vaultStateKey.as_ref(),
            &[ctx.accounts.vault_state.vault_bump]
        ]];
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
        &signer_seeds,
        );

        transfer(cpi_ctx, balance)?;

        Ok(())
    }
    //Close
    pub fn close(ctx: Context<Close>) -> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        init,
        seeds = [b"state".as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = VaultAccount::DISCRIMINATOR.len() + VaultAccount::INIT_SPACE,
    )]
    pub vault_state : Account<'info, VaultAccount>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
//deposit
#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault : SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program: Program<'info, System>,
}

//Withdraw
#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault : SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program: Program<'info, System>,
}

//Close
#[derive(Accounts)]
pub struct Close<'info>{
    #[account(
        mut,
        seeds = [b"vault",user.key().as_ref()],
        close = user,
        bump
    )]
    pub vault : Account<'info,VaultAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(InitSpace)]
#[account]
pub struct VaultAccount {
    pub vault_bump: u8,
    pub state_bump: u8,
}
