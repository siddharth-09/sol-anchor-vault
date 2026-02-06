use anchor_lang::prelude::*;

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
        let vault_account = &mut ctx.accounts.vault;
        vault_account.vault_authority = ctx.accounts.user.key();
        Ok(())
    }
    //Deposit
    pub fn deposit(ctx: Context<Deposit>,amount : u64)-> Result<()>{
        let vault = &ctx.accounts.vault;
        let user = &mut ctx.accounts.user;
        if vault.vault_authority != user.key(){
            return Err(ErrorCode::Unauthorized.into());
        }
        if user.lamports() < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }
        let ix = anchor_lang::solana_program::system_instruction::transfer(
                user.key,
                &vault.key(),
                amount
            );
        anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    user.to_account_info(),
                    vault.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ])?;
            Ok(())
    }
    //Withdraw
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()>{
        let vault = &ctx.accounts.vault;
        let user = &ctx.accounts.user;
        let balance = vault.to_account_info().lamports();
        if vault.vault_authority != user.key(){
            return Err(ErrorCode::Unauthorized.into());
        }
        if vault.to_account_info().lamports() == 0{
            return Err(ErrorCode::VaultInsufficientBalance.into());
        }
        **vault.to_account_info().try_borrow_mut_lamports()? -= balance;
        **user.to_account_info().try_borrow_mut_lamports()? +=balance;
        // let ix = anchor_lang::solana_program::system_instruction::transfer(
        //     &vault.key(),
        //     user.key,
        //     balance
        // );
        // anchor_lang::solana_program::program::invoke_signed(&ix, 
        //     &[
        //         user.to_account_info(),
        //         vault.to_account_info(),

        //     ],
        // signer)?;
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
        seeds = [b"vault".as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32,
    )]
    pub vault : Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>,
}
//deposit
#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(
        mut,
        seeds = [b"vault",user.key().as_ref()],
        bump,
    )]
    pub vault : Account<'info,VaultAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program: Program<'info, System>,
}

//Withdraw
#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(
        mut,
        seeds = [b"vault",user.key().as_ref()],
        bump
    )]
    pub vault : Account<'info,VaultAccount>,
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



#[account]
pub struct VaultAccount {
    vault_authority : Pubkey
}
